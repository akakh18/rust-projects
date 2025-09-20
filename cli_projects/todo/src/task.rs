use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::manager::get_tasks_file;

pub struct Task {
    pub name: String,
    pub file_path: PathBuf,
}

impl Task {
    pub fn new(name: &str) -> Self {
        let file_path = get_tasks_file(name);
        Task {
            name: name.to_string(),
            file_path,
        }
    }

    /// Create a new empty task file
    pub fn create(&self) {
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)
            .expect("Failed to create task file");
    }

    /// Append a description to the task
    pub fn add_description(&self, desc: &str) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_path)
            .expect("Failed to open task file");
        writeln!(file, "{}", desc).expect("Failed to write description");
    }

    /// Show all lines of a task file
    pub fn show(&self) {
        if !self.file_path.exists() {
            println!("Task '{}' does not exist", self.name);
            return;
        }

        let file = fs::File::open(&self.file_path).expect("Failed to open task file");
        let reader = BufReader::new(file);

        println!("Task '{}':", self.name);
        for (i, line) in reader.lines().enumerate() {
            let line = line.expect("Failed to read line");
            println!("-> {}: {}", i + 1, line);
        }
    }
}
