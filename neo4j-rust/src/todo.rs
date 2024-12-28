use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

impl TodoStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TodoStatus::Pending => "PENDING",
            TodoStatus::InProgress => "IN_PROGRESS",
            TodoStatus::Completed => "COMPLETED",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "PENDING" => Ok(TodoStatus::Pending),
            "IN_PROGRESS" => Ok(TodoStatus::InProgress),
            "COMPLETED" => Ok(TodoStatus::Completed),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}