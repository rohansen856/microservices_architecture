
## Testing the Neo4j Todo App

### 1. Pre-requisites
- Ensure the application is running.
- Use a REST client like Postman, cURL, or HTTPie.

### 2. Test Endpoints

#### a. Get All Todos
**Request**:
```bash
curl -X GET http://127.0.0.1:8080/todo
```
**Expected Response**:
```json
[
  {
    "id": 1,
    "title": "Test Todo",
    "description": "This is a test",
    "status": "PENDING",
    "created_at": "2024-12-29T12:34:56Z",
    "updated_at": "2024-12-29T12:34:56Z"
  }
]
```

#### b. Get Todo by ID
**Request**:
```bash
curl -X GET http://127.0.0.1:8080/todo/1
```
**Expected Response**:
```json
{
  "id": 1,
  "title": "Test Todo",
  "description": "This is a test",
  "status": "PENDING",
  "created_at": "2024-12-29T12:34:56Z",
  "updated_at": "2024-12-29T12:34:56Z"
}
```

#### c. Create a Todo
**Request**:
```bash
curl -X POST http://127.0.0.1:8080/todo \
-H "Content-Type: application/json" \
-d '{"title": "New Todo", "description": "Todo description", "status": "PENDING"}'
```
**Expected Response**:
```json
{
  "id": 2,
  "title": "New Todo",
  "description": "Todo description",
  "status": "PENDING",
  "created_at": "2024-12-29T12:45:00Z",
  "updated_at": "2024-12-29T12:45:00Z"
}
