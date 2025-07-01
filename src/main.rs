mod adapter;
mod domain;
mod infrastructure;
mod usecase;

use adapter::cli::{CliArgs, TaskCommand};
use clap::Parser;
use infrastructure::storage::{load_tasks, save_tasks};
use usecase::task_manager::*;

fn main() {
    let args = CliArgs::parse();
    let mut tasks = load_tasks().unwrap_or_default();

    match args.command {
        TaskCommand::Add { description } => {
            add_task(&mut tasks, description);
            save_tasks(&tasks).expect("Failed to save tasks");
            println!("✅ Task added!");
        }

        TaskCommand::List => {
            list_tasks(&tasks);
        }

        TaskCommand::Done { index } => match mark_task_done(&mut tasks, index - 1) {
            Ok(_) => {
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("✅ Task marked as done!");
            }
            Err(e) => println!("❌ {}", e),
        },

        TaskCommand::Delete { index } => match delete_task(&mut tasks, index - 1) {
            Ok(_) => {
                save_tasks(&tasks).expect("Failed to save tasks");
                println!("✅ Task deleted!");
            }
            Err(e) => println!("❌ {}", e),
        },
    }
}
