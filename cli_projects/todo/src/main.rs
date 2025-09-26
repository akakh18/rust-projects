mod task;
mod manager;

use crate::task::Task;
use crate::manager::list_tasks;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: todo <command> [arguments]");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Please provide a task name");
            } else {
                let task = Task::new(&args[2]);
                task.create();
                println!("Added task: {}", task.name);
            }
        }
        "list" => {
            list_tasks();
        }
        "desc" => {
            if args.len() < 4 {
                println!("Usage: todo desc <task> <description>");
            } else {
                let task = Task::new(&args[2]);
                task.add_description(&args[3]);
                println!("Added description to task: {}", task.name);
            }
        }
        "show" => {
            if args.len() < 3 {
                println!("Please provide a task to show");
            } else {
                let task = Task::new(&args[2]);
                task.show();
            }
        }
        "resolve" => {
            if args.len() < 4 {
                println!("Usage: todo resolve <task> <line_number>");
            } else {
                let task = Task::new(&args[2]);
                let line_number: usize = match args[3].parse() {
                    Ok(n) => n,
                    Err(_) => {
                        println!("Invalid line number: {}", args[3]);
                        return;
                    }
                };
                task.resolve(line_number);
                println!("Marked line {} as resolved in task '{}'", line_number, task.name);
            }
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
}
