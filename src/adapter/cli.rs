use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Task Manager CLI",
    version = "1.0",
    author = "Md Mostafa Akash",
    about = "Manage your tasks from the terminal"
)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: TaskCommand,
}

#[derive(Subcommand)]
pub enum TaskCommand {
    Add {
        description: String,
    },

    List,

    Done {
        // Task number(1-based index)
        index: usize,
    },

    Delete {
        // Task number(1-based index)
        index: usize,
    },
}
