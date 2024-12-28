# Testing the Todo App

Follow these steps to test the endpoints of the Todo App:

## Prerequisites
- Ensure the app is running locally at `http://127.0.0.1:8080`.
- Use a tool like [Postman](https://www.postman.com/) or `curl` for testing.

## Endpoints

### 1. Get All Todos
**Endpoint**: `GET /todo`  
**Description**: Fetches all todo items.  

**Example Request**:
```bash
curl -X GET http://127.0.0.1:8080/todo
```

### 2. Create a Todos
**Endpoint**: `POST /todo`  
**Description**: Create a todo item.  

**Example Request**:
```bash
curl -X POST http://127.0.0.1:8080/todo \
-H "Content-Type: application/json" \
-d '{"title": "Learn Rust", "completed": false}'
```

### 2. Get Todo by id
**Endpoint**: `GET /todo/{:id}`  
**Description**: Fetches a todo item.  

**Example Request**:
```bash
curl -X GET http://127.0.0.1:8080/todo/1
```

## Expected Responses
- 200 OK: Successfully fetched todos or a specific todo.
- 201 Created: Successfully created a new todo.
- 404 Not Found: Todo with the specified ID does not exist.
- 500 Internal Server Error: Issues with the server or database.
