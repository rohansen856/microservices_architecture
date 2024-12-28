## Todo App (MongoDB + Rust)

A simple Todo application built using **Actix Web** and **MongoDB**.

### Features
- Create a new Todo.
- Retrieve all Todos.
- Retrieve a Todo by its ID.

### Routes
1. **GET /todo**
   - Fetch all Todos.
2. **POST /todo**
   - Create a new Todo.
   - Example JSON body:
     ```json
     {
         "title": "My new task",
         "completed": false
     }
     ```
3. **GET /todo/{id}**
   - Fetch a Todo by its unique ID.
