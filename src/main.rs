mod domain;

use domain::task::{Task, TaskStatus};
use uuid::Uuid;
use chrono::Utc;

fn main() {
    let task = Task {
        id: Uuid::new_v4(),
        description: "Learn Rust".to_string(),
        status: TaskStatus::Pending,
        created_at: Utc::now(),
    };

    println!("{:#?}", task);
}
