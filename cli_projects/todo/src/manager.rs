use std::env;
use std::fs;
use std::path::PathBuf;

pub fn get_base_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current exe path");
    let base_dir = exe_path.parent().unwrap().join("lib");
    fs::create_dir_all(&base_dir).expect("Failed to create lib directory");
    base_dir
}

pub fn get_tasks_file(name: &str) -> PathBuf {
    get_base_path().join(name)
}

pub fn list_tasks() {
    let lib_path = get_base_path();

    if !lib_path.exists() {
        println!("lib directory does not exist");
        return;
    }

    match fs::read_dir(&lib_path) {
        Ok(entries) => {
            println!("Tasks in lib:");
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    println!("File: {}", path.file_name().unwrap().to_string_lossy());
                }
            }
        }
        Err(e) => println!("Failed to read lib directory: {}", e),
    }
}
