mod domain;
mod infrastructure;
mod usecase;
mod adapter;

use infrastructure::storage::{load_tasks, save_tasks};
use usecase::task_manager::*;
use adapter::cli::{CliArgs, TaskCommand};
use clap::Parser;
use colored::*;

fn main() {
    let args = CliArgs::parse();
    let mut tasks = load_tasks().unwrap_or_default();

    match args.command {
        TaskCommand::Add { description } => {
            add_task(&mut tasks, description);
            save_tasks(&tasks).expect("Failed to save tasks");
            println!("{}", "✅ Task added!".green());
        }

        TaskCommand::List => {
            list_tasks(&tasks);
        }

        TaskCommand::Done { index } => {
            match mark_task_done(&mut tasks, index - 1) {
                Ok(_) => {
                    save_tasks(&tasks).expect("Failed to save tasks");
                    println!("{}", "✅ Task marked as done!".green());
                }
                Err(e) => println!("{}", format!("❌ {}", e).red()),
            }
        }

        TaskCommand::Delete { index } => {
            match delete_task(&mut tasks, index - 1) {
                Ok(_) => {
                    save_tasks(&tasks).expect("Failed to save tasks");
                    println!("{}", "❌ Task deleted!".red());
                }
                Err(e) => println!("{}", format!("❌ {}", e).red()),
            }
        }   
    }
}
