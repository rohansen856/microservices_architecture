
## Setup Instructions

### Prerequisites
- **Rust** (latest version)
- **Cargo** (Rust package manager)
- **Docker** & **Docker Compose**
- MongoDB URI in `.env` file

### Steps
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```

2. Copy `.env.example` to `.env`:
   ```bash
   cp .env.example .env
   ```

3. Update `.env` with your MongoDB configuration:
   ```env
   MONGO_URI=mongodb://<username>:<password>@localhost:27017/todo_db
   ```

4. Start Docker:
   ```bash
   docker-compose up -d
   ```

5. Run the application:
   ```bash
   cargo run
   ```

6. Access the application:
   - Base URL: `http://127.0.0.1:8080`
   - Example endpoints:
     - `GET /todo`
     - `POST /todo`
     - `GET /todo/{id}`
