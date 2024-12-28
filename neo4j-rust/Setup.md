
## Prerequisites
- Docker & Docker Compose installed.
- Rust and Cargo installed.
- `.env` file for environment variables.

### Environment Variables
Create a `.env` file in the root directory with the following content:
```env
# Neo4j Configuration
NEO4J_URI="neo4j://localhost:7687"
NEO4J_USER="neo4j"
NEO4J_PASSWORD="yourSecurePassword123"

# Server Configuration
HOST="127.0.0.1"
PORT="8080"

# Logging Configuration (optional)
RUST_LOG="debug"
```

## Steps to Set Up

### 1. Start Neo4j with Docker Compose
Ensure your `docker-compose.yaml` is running with the following command:
```bash
docker-compose up -d
```

### 2. Install Rust Dependencies
Install Rust dependencies by running:
```bash
cargo build
```

### 3. Run the Application
Start the server:
```bash
cargo run
```
The server will be accessible at `http://127.0.0.1:8080`.
