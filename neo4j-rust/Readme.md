## Neo4j Todo App

This is a Rust-based web application for managing a Todo list using Neo4j as the database. The app provides APIs to create, retrieve, and fetch Todos by ID.

## Features
- RESTful APIs for managing Todos.
- Uses Neo4j graph database for data storage.
- Follows Actix-web framework for handling HTTP requests.
- Todo items have attributes like `title`, `description`, `status`, and timestamps for `created_at` and `updated_at`.

## Endpoints

### 1. Get all Todos
**URL**: `/todo`
**Method**: `GET`
**Response**: JSON array of Todos.

### 2. Get Todo by ID
**URL**: `/todo/{id}`
**Method**: `GET`
**Response**: JSON object of the requested Todo.

### 3. Create a Todo
**URL**: `/todo`
**Method**: `POST`
**Request Body**:
```json
{
  "title": "Sample Title",
  "description": "Sample Description",
  "status": "PENDING"
}
```
**Response**: JSON object of the created Todo.
