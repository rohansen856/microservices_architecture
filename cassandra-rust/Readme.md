## Cassandra-based Todo App

This project implements a simple Todo application using Cassandra as the database and Actix Web as the web framework. It includes endpoints to create, fetch, update, and list todos.

### Features

- **Endpoints**:
  - `GET /todo`: Fetch all todos.
  - `GET /todo/{id}`: Fetch a todo by its UUID.
  - `POST /todo`: Create a new todo.
  - `PATCH /patch/{id}`: Update a todo.
- **Cassandra Integration**: Efficiently manages todos using a Cassandra database.
- **Logging**: Uses `env_logger` for logging errors and application events.

### Prerequisites

- Rust and Cargo installed.
- Docker installed (for setting up Cassandra).
- A running Cassandra database instance.
