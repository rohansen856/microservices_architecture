use actix_web::{web, App, HttpServer, HttpResponse, Error};
use mongodb::{Client, Collection};
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    completed: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTodoRequest {
    title: String,
}

struct AppState {
    todo_collection: Collection<Todo>,
}

async fn get_all_todos(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let todos = data
        .todo_collection
        .find(None, None)
        .await
        .expect("Failed to fetch todos");
    
    let todos: Vec<Todo> = todos
        .try_collect()
        .await
        .expect("Failed to collect todos");

    Ok(HttpResponse::Ok().json(todos))
}

async fn create_todo(
    data: web::Data<AppState>,
    todo_req: web::Json<CreateTodoRequest>,
) -> Result<HttpResponse, Error> {
    let new_todo = Todo {
        id: None,
        title: todo_req.title.clone(),
        completed: false,
        created_at: chrono::Utc::now(),
    };

    let result = data
        .todo_collection
        .insert_one(new_todo, None)
        .await
        .expect("Failed to create todo");

    Ok(HttpResponse::Created().json(result.inserted_id))
}

async fn get_todo_by_id(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let obj_id = ObjectId::parse_str(id.as_str()).expect("Invalid id");
    let filter = bson::doc! {"_id": obj_id};

    match data.todo_collection.find_one(filter, None).await {
        Ok(Some(todo)) => Ok(HttpResponse::Ok().json(todo)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let mongodb_uri = env::var("MONGODB_URI")
        .expect("MONGODB_URI must be set");
    let database_name = env::var("DATABASE_NAME")
        .expect("DATABASE_NAME must be set");

    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");
    
    let db = client.database(&database_name);
    let todo_collection = db.collection::<Todo>("todos");

    let app_state = web::Data::new(AppState {
        todo_collection,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todo", web::get().to(get_all_todos))
            .route("/todo", web::post().to(create_todo))
            .route("/todo/{id}", web::get().to(get_todo_by_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
