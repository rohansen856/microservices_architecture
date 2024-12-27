use cassandra_cpp::{AsRustType, Cluster, LendingIterator, Session};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: String,
    pub completed: bool,
}

pub struct CassandraDb {
    session: Arc<Session>,
}

impl CassandraDb {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut cluster = Cluster::default();
        // cluster.set_credentials(user, password)?;
        cluster.set_contact_points("127.0.0.1")?;
        let session = Arc::new(cluster.connect().await?);

        // Create keyspace
        let create_keyspace = "CREATE KEYSPACE IF NOT EXISTS todo_keyspace \
                              WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1}";
        session.execute(create_keyspace).await?;

        // Create table
        let create_table = "CREATE TABLE IF NOT EXISTS todo_keyspace.todos (\
                           id uuid PRIMARY KEY,\
                           title text,\
                           completed boolean)";
        session.execute(create_table).await?;

        Ok(CassandraDb { session })
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
        let query = "SELECT id, title, completed FROM todo_keyspace.todos";
        
        let result = self.session.execute(query).await?;
        
        let mut todos = Vec::new();

        let mut iter = result.iter();
        while let Some(row) = iter.next() {
            let id: Uuid = row.get_by_name("id")?;
            let title: String = row.get_by_name("title")?;
            let completed: bool = row.get_by_name("completed")?;
    
            // Create an Employee instance and add it to the vector
            todos.push(Todo { id, title, completed });
        }
    
        // Return the vector of todos

        Ok(todos)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Todo>, Box<dyn std::error::Error>> {
        let query = "SELECT id, title, completed FROM todo_keyspace.todos WHERE id = ?";
        let mut statement = self.session.statement(query);
        statement.bind_uuid(0, id.try_into().expect("invalid uuid")).unwrap();
        
        let result = statement.execute().await?;
        
        if let Some(row) = result.first_row() {
            Ok(Some(Todo {
                id: row.get_by_name("id")?,
                title: row.get_by_name("title")?,
                completed: row.get_by_name("completed")?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn create(&self, todo: CreateTodo) -> Result<Todo, Box<dyn std::error::Error>> {
        let id = Uuid::new_v4();
        let query = "INSERT INTO todo_keyspace.todos (id, title, completed) VALUES (?, ?, ?)";
        let mut statement = self.session.statement(query);
        
        statement.bind_uuid(0, id.try_into().expect("invalid uuid"))?;
        statement.bind_string(1, todo.title.as_str())?;
        statement.bind_bool(2, false)?;
        
        statement.execute().await?;
        
        Ok(Todo {
            id,
            title: todo.title,
            completed: false,
        })
    }

    pub async fn update(&self, id: Uuid, updates: UpdateTodo) -> Result<Todo, Box<dyn std::error::Error>> {
        // Define the CQL query to update an employee's details
        let query = "UPDATE todo_keyspace.todos SET title = ? , completed = ? WHERE id = ?";
        let mut statement = self.session.statement(query);
        
        statement.bind_string(1, updates.title.as_str())?;
        statement.bind_bool(2, updates.completed)?;
        statement.bind_uuid(0, id.try_into().expect("invalid uuid"))?;
        
        statement.execute().await?;
        
        Ok(Todo {
            id,
            title: updates.title,
            completed: updates.completed,
        })
    }
}
