use actix_web::{web, App, HttpServer, Responder, HttpResponse, Result};
use sqlx::PgPool;
use std::env;
use chrono::Utc;

mod todo;
use todo::{Todo, NewTodo};

async fn get_all_todos(db_pool: web::Data<PgPool>) -> Result<impl Responder> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(db_pool.get_ref())
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to retrieve todos: {}", e.to_string()))
        })?;
    Ok(HttpResponse::Ok().json(todos))
}

async fn create_todo(
    db_pool: web::Data<PgPool>,
    new_todo: web::Json<NewTodo>,
) -> Result<impl Responder> {
    let new_todo = new_todo.into_inner();
    let completed = new_todo.completed.unwrap_or(false);

    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(new_todo.title)
    .bind(completed)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to create todo: {}", e.to_string()))
    })?;

    Ok(HttpResponse::Created().json(todo))
}

async fn get_todo_by_id(
    db_pool: web::Data<PgPool>,
    todo_id: web::Path<i32>,
) -> Result<impl Responder> {
    let id = todo_id.into_inner();
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(db_pool.get_ref())
        .await
        .map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("Failed to retrieve todo: {}", e.to_string()))
        })?;

    if let Some(todo) = todo {
        Ok(HttpResponse::Ok().json(todo))
    } else {
        Ok(HttpResponse::NotFound().body("Todo not found"))
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = format!(
        "postgres://{}:{}@localhost/{}",
        env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set"),
        env::var("POSTGRES_DB").expect("POSTGRES_DB must be set")
    );

    let db_pool = PgPool::connect(&database_url).await.expect("Failed to connect to the database");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT false,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
        )"
    )
    .execute(&db_pool)
    .await
    .expect("Failed to create todos table");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .route("/todo", web::get().to(get_all_todos))
            .route("/todo", web::post().to(create_todo))
            .route("/todo/{id}", web::get().to(get_todo_by_id))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
