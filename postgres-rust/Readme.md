# Todo App

A simple Rust-based Todo App with RESTful APIs using Actix Web and SQLx for PostgreSQL integration.

## Features
- Create a new todo.
- Fetch all todos.
- Fetch a specific todo by ID.
- Tracks creation and update timestamps.

## Prerequisites
- Rust (latest stable version)
- Docker and Docker Compose
- PostgreSQL database

## Technologies Used
- **Actix Web**: Web framework for building REST APIs.
- **SQLx**: Async PostgreSQL ORM.
- **Chrono**: Handling timestamps.
- **dotenvy**: Loading environment variables.

## Endpoints
1. `GET /todo` - Get all todos.
2. `POST /todo` - Create a new todo.
3. `GET /todo/{id}` - Get a specific todo by its ID.
