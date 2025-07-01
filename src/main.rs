mod domain;
mod infrastructure;
mod usecase;

use domain::task::Task;
use infrastructure::storage::{load_tasks, save_tasks};
use usecase::task_manager::{add_task, delete_task, list_tasks, mark_task_done};

fn main() {
    let mut tasks: Vec<Task> = load_tasks().unwrap_or_default();

    println!("📋 Current Tasks:");
    list_tasks(&tasks);

    println!("\n➕ Adding a new task...");
    add_task(&mut tasks, "Read the rust book".into());

    println!("\n✅ Marking task 1 as done...");
    if let Err(e) = mark_task_done(&mut tasks, 0) {
        println!("❌ {}", e);
    }

    println!("\n❌ Deleting task 1...");
    if let Err(e) = delete_task(&mut tasks, 0) {
        println!("❌ {}", e);
    }

    println!("\n💾 Saving...");
    save_tasks(&tasks).unwrap();

    println!("\n📋 Updated Tasks:");
    list_tasks(&tasks);
}
