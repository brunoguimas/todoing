use std::fs;
use std::io::Read;

use crate::task::*;

pub fn save_tasks(tasks: Vec<Task>) {
    let tasks_json = serde_json::to_string_pretty(&tasks).unwrap();

    match fs::write("tasks.json", tasks_json) {
        Ok(_) => println!("Tasks saved with success"),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1)
        }
    }
}

pub fn load_tasks() -> Vec<Task> {
    match fs::File::open("tasks.json") {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content).unwrap();
            serde_json::from_str(&content).unwrap()
        }
        Err(_) => Vec::new(),
    }
}
