## Local Setup

### Step 1: Clone the repository
```bash
$ git clone <repository-url>
$ cd cassandra-todo-app
```

### Step 2: Set up `.env` file
Copy the provided `.env.example` file to `.env` and update the configurations as needed.
```bash
$ cp .env.example .env
```

### Step 3: Run Cassandra with Docker
Ensure you have Docker installed. Start Cassandra using:
```bash
$ docker compose up
```

### Step 4: Initialize Database
Create the necessary tables in Cassandra. Use the provided `schema.cql` file:
```bash
$ docker exec -it cassandra cqlsh
> SOURCE '/path/to/schema.cql';
```

### Step 5: Run the Application
```bash
$ cargo run
```
The server will start on `127.0.0.1:8080`.
