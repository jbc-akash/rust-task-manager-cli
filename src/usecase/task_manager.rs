use crate::domain::task::{Task, TaskStatus};
use chrono::{Utc, Local};
use uuid::Uuid;
use colored::*;

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
        println!("{}", "📭 No tasks found.".yellow());
        return;
    }

    println!("{}", "📝 Your Tasks:".bold().underline());

    for (i, task) in tasks.iter().enumerate() {
        let index = format!("{:>2}.", i+1).cyan();
        
        let status = match task.status {
            TaskStatus::Pending => "🕒 Pending".normal(),
            TaskStatus::Done => "✅ Done".green(),
        };

        let created_at = task
            .created_at
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M")
            .to_string();

         println!(
            "{} {}  {}  [{}]",
            index,
            task.description.bold(),
            status,
            created_at.dimmed()
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
