
### Step 1: Start the Application and container
Make sure the server is running:
```bash
$ docker compose up -d
$ cargo run
```

### Step 2: Use API Client
Use `curl`, `Postman`, or any other API client to test the endpoints.

#### Fetch All Todos
```bash
$ curl -X GET http://127.0.0.1:8080/todo
```

#### Fetch Todo by ID
```bash
$ curl -X GET http://127.0.0.1:8080/todo/{id}
```
Replace `{id}` with a valid UUID.

#### Create a New Todo
```bash
$ curl -X POST http://127.0.0.1:8080/todo \
  -H "Content-Type: application/json" \
  -d '{"title": "New Task", "completed": false}'
```

#### Update a Todo
```bash
$ curl -X PATCH http://127.0.0.1:8080/patch/{id} \
  -H "Content-Type: application/json" \
  -d '{"title": "Updated Task", "completed": true}'
```

### Step 3: Check Logs
Logs will appear in the console. Ensure `env_logger` is correctly set up for better debugging.

---

## Notes
- Ensure the Cassandra instance is running before testing.
- All UUIDs must be valid; invalid UUIDs will result in `400 Bad Request` errors.
