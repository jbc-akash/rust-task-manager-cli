use task_manager_cli::domain::task::{Task, TaskStatus};
use task_manager_cli::usecase::task_manager::*;
use uuid::Uuid;
use chrono::Utc;

fn sample_task(description: &str) -> Task {
    Task {
        id: Uuid::new_v4(),
        description: description.to_string(),
        status: TaskStatus::Pending,
        created_at: Utc::now(),
    }
}

#[test]
fn test_add_task() {
    let mut tasks = vec![];
    add_task(&mut tasks, "Test task".to_string());
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].description, "Test task");
}

#[test]
fn test_mark_task_done() {
    let mut tasks = vec![sample_task("Test sample task 1")];
    let result = mark_task_done(&mut tasks, 0);
    assert!(result.is_ok());
    assert_eq!(tasks[0].status, TaskStatus::Done);
}

#[test]
fn test_mark_task_done_invalid_index() {
    let mut tasks = vec![];
    let result = mark_task_done(&mut tasks, 0);
    assert!(result.is_err());
}

#[test]
fn test_delete_task() {
    let mut tasks = vec![sample_task("Temporary tasks")];
    let result = delete_task(&mut tasks, 0);
    assert!(result.is_ok());
    assert!(tasks.is_empty());
}

#[test]
fn test_delete_task_invalid_index() {
    let mut tasks = vec![];
    let result = delete_task(&mut tasks, 1);
    assert!(result.is_err());
}