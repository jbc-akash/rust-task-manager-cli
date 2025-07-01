mod domain;
mod infrastructure;

use chrono::Utc;
use domain::task::{Task, TaskStatus};
use infrastructure::storage::{load_tasks, save_tasks};
use uuid::Uuid;

fn main() {
    let mut tasks = load_tasks().expect("Failed to load tasks");

    let new_task = Task {
        id: Uuid::new_v4(),
        description: "Learn Rust".to_string(),
        status: TaskStatus::Pending,
        created_at: Utc::now(),
    };

    tasks.push(new_task);
    save_tasks(&tasks).expect("Failed to save tasks");

    println!("✅ Saved {} task(s).", tasks.len());
}
