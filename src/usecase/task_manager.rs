use crate::domain::task::{Task, TaskStatus};
use chrono::Utc;
use uuid::Uuid;

pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let task = Task {
        id: Uuid::new_v4(),
        description,
        status: TaskStatus::Pending,
        created_at: Utc::now(),
    };

    tasks.push(task);
}

pub fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }

    for (i, task) in tasks.iter().enumerate() {
        let status = match task.status {
            TaskStatus::Pending => "🕒",
            TaskStatus::Done => "✅",
        };

        println!(
            "{}. {} [{}] - {}",
            i + 1,
            task.description,
            status,
            task.created_at.to_rfc3339()
        );
    }
}

pub fn mark_task_done(tasks: &mut Vec<Task>, index: usize) -> Result<(), String> {
    if let Some(task) = tasks.get_mut(index) {
        task.status = TaskStatus::Done;
        Ok(())
    } else {
        Err(format!("Task with index {} not found.", index + 1))
    }
}

pub fn delete_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), String> {
    if index < tasks.len() {
        tasks.remove(index);
        Ok(())
    } else {
        Err(format!("Task with index {} not found.", index + 1))
    }
}
