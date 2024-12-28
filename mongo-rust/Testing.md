
## Testing the Application

### Using `curl`
1. **Fetch all Todos:**
   ```bash
   curl -X GET http://127.0.0.1:8080/todo
   ```

2. **Create a new Todo:**
   ```bash
   curl -X POST http://127.0.0.1:8080/todo \
     -H "Content-Type: application/json" \
     -d '{"title": "Test Task", "completed": false}'
   ```

3. **Fetch a Todo by ID:**
   ```bash
   curl -X GET http://127.0.0.1:8080/todo/<id>
   ```

### Using Postman
1. Open Postman.
2. Create a new collection and add requests for the endpoints.
3. Use the example JSON body provided for POST requests.

### Notes
- Ensure MongoDB is running and accessible via the URI in `.env`.
- Check Docker logs if the service is not running properly:
  ```bash
  docker-compose logs
  
