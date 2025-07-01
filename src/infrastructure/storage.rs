use crate::domain::task::Task;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;

const TASK_FILE: &str = "tasks.json";

pub fn load_tasks() -> io::Result<Vec<Task>> {
    let path = Path::new(TASK_FILE);

    if !path.exists() {
        return Ok(vec![]);
    }

    let file = File::open(TASK_FILE)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(TASK_FILE)?;

    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tasks)?;
    Ok(())
}
