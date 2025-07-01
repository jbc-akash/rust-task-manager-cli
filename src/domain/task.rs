use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
}
