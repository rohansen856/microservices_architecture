use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use neo4rs::{Graph, Query};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i64>,
    title: String,
    description: String,
    status: TodoStatus,
    #[serde(default = "Utc::now")]
    created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

impl TodoStatus {
    fn as_str(&self) -> &'static str {
        match self {
            TodoStatus::Pending => "PENDING",
            TodoStatus::InProgress => "IN_PROGRESS",
            TodoStatus::Completed => "COMPLETED",
        }
    }

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "PENDING" => Ok(TodoStatus::Pending),
            "IN_PROGRESS" => Ok(TodoStatus::InProgress),
            "COMPLETED" => Ok(TodoStatus::Completed),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

struct AppState {
    graph: Arc<Graph>,
}

async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    let query = Query::new(
        "MATCH (t:Todo) 
         RETURN ID(t) as id, 
                t.title as title, 
                t.description as description,
                t.status as status,
                t.created_at as created_at,
                t.updated_at as updated_at
         ORDER BY t.created_at DESC".to_string()
    );

    let mut result = match data.graph.execute(query).await {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Query error: {}", e)),
    };

    let mut todos = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let status_str: String = row.get("status").unwrap();
        let status = match TodoStatus::from_str(&status_str) {
            Ok(s) => s,
            Err(e) => return HttpResponse::InternalServerError().json(e),
        };

        let todo = Todo {
            id: Some(row.get("id").unwrap()),
            title: row.get("title").unwrap(),
            description: row.get("description").unwrap(),
            status,
            created_at: row.get("created_at").unwrap(),
            updated_at: row.get("updated_at").unwrap(),
        };
        todos.push(todo);
    }

    HttpResponse::Ok().json(todos)
}

async fn get_todo_by_id(path: web::Path<i64>, data: web::Data<AppState>) -> impl Responder {
    let id = path.into_inner();
    let query = Query::new(
        "MATCH (t:Todo) 
         WHERE ID(t) = $id 
         RETURN ID(t) as id, 
                t.title as title, 
                t.description as description,
                t.status as status,
                t.created_at as created_at,
                t.updated_at as updated_at".to_string()
    )
    .param("id", id);

    let mut result = match data.graph.execute(query).await {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Query error: {}", e)),
    };

    match result.next().await {
        Ok(Some(row)) => {
            let status_str: String = row.get("status").unwrap();
            let status = match TodoStatus::from_str(&status_str) {
                Ok(s) => s,
                Err(e) => return HttpResponse::InternalServerError().json(e),
            };

            let todo = Todo {
                id: Some(row.get("id").unwrap()),
                title: row.get("title").unwrap(),
                description: row.get("description").unwrap(),
                status,
                created_at: row.get("created_at").unwrap(),
                updated_at: row.get("updated_at").unwrap(),
            };
            HttpResponse::Ok().json(todo)
        }
        Ok(None) => HttpResponse::NotFound().json("Todo not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e)),
    }
}

async fn create_todo(todo: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let now = Utc::now();
    let query = Query::new(
        "CREATE (t:Todo {
            title: $title,
            description: $description,
            status: $status,
            created_at: $created_at,
            updated_at: $updated_at
         })
         RETURN ID(t) as id, 
                t.title as title, 
                t.description as description,
                t.status as status,
                t.created_at as created_at,
                t.updated_at as updated_at".to_string()
    )
    .param("title", todo.title.as_str())
    .param("description", todo.description.as_str())
    .param("status", todo.status.as_str())
    .param("created_at", now.to_string())
    .param("updated_at", now.to_string());

    let mut result = match data.graph.execute(query).await {
        Ok(result) => result,
        Err(e) => return HttpResponse::InternalServerError().json(format!("Query error: {}", e)),
    };

    match result.next().await {
        Ok(Some(row)) => {
            let status_str: String = row.get("status").unwrap();
            let status = match TodoStatus::from_str(&status_str) {
                Ok(s) => s,
                Err(e) => return HttpResponse::InternalServerError().json(e),
            };

            let created_todo = Todo {
                id: Some(row.get("id").unwrap()),
                title: row.get("title").unwrap(),
                description: row.get("description").unwrap(),
                status,
                created_at: row.get("created_at").unwrap(),
                updated_at: row.get("updated_at").unwrap(),
            };
            HttpResponse::Created().json(created_todo)
        }
        Ok(None) => HttpResponse::InternalServerError().json("Failed to create todo"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let neo4j_uri = env::var("NEO4J_URI").expect("NEO4J_URI must be set");
    let neo4j_user = env::var("NEO4J_USER").expect("NEO4J_USER must be set");
    let neo4j_password = env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD must be set");
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set")
        .parse::<u16>()
        .expect("PORT must be a number");

    let graph = Arc::new(
        Graph::new(&neo4j_uri, &neo4j_user, &neo4j_password)
            .await
            .expect("Failed to create Neo4j connection")
    );

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                graph: graph.clone(),
            }))
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::get().to(get_todo_by_id))
            .route("/todo", web::post().to(create_todo))
    })
    .bind((host, port))?
    .run()
    .await
}