
// src/main.rs
mod db;

use actix_web::{web, App, HttpResponse, HttpServer};
use db::{CassandraDb, CreateTodo, UpdateTodo};
use std::sync::Arc;
use uuid::Uuid;
use log::error;

struct AppState {
    db: Arc<CassandraDb>,
}

async fn get_todos(state: web::Data<AppState>) -> HttpResponse {
    match state.db.get_all().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => {
            error!("Failed to fetch todos: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_todo_by_id(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
) -> HttpResponse {
    match state.db.get_by_id(id.into_inner()).await {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("Failed to fetch todo: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn create_todo(
    state: web::Data<AppState>,
    todo: web::Json<CreateTodo>,
) -> HttpResponse {
    match state.db.create(todo.into_inner()).await {
        Ok(new_todo) => HttpResponse::Created().json(new_todo),
        Err(e) => {
            error!("Failed to create todo: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn update_todo(
    state: web::Data<AppState>,
    id: web::Path<Uuid>,
    update: web::Json<UpdateTodo>,
) -> HttpResponse {
    match state.db.update(id.into_inner(), update.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("Failed to update todo: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // Initialize database
    let db = Arc::new(CassandraDb::new().await.expect("Failed to initiate cassandra connection"));


    // Start HTTP server
    let state = web::Data::new(AppState { db: db.clone() });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/get", web::get().to(get_todos))
            .route("/get/{id}", web::get().to(get_todo_by_id))
            .route("/post", web::post().to(create_todo))
            .route("/patch/{id}", web::patch().to(update_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}