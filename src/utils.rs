use std::fs;

use crate::storage::*;
use crate::task::Task;
use crate::task::TaskStatus;
use chrono::Local;
use prettytable::row;
use prettytable::table;

pub fn add_task(description: String) {
    let mut tasks = load_tasks();
    let id = tasks.iter().map(|t| t.id).max().unwrap_or_default() + 1;

    let new_task = Task::new(id, description);
    tasks.push(new_task);

    save_tasks(tasks);
}

pub fn remove_task(id: &u32) {
    let mut tasks = load_tasks();

    let target = tasks.iter().enumerate().find(|t| &t.1.id == id);

    if let None = target {
        eprintln!("Error: Task not found: {}", id);
        std::process::exit(1)
    }

    let i = target.unwrap().0;

    tasks.remove(i);

    save_tasks(tasks);
}

pub fn update_task(id: &u32, description: &str) {
    let mut tasks = load_tasks();

    let target = tasks.iter().enumerate().find(|t| &t.1.id == id);

    if let None = target {
        eprintln!("Error: Task not found: {}", id);
        std::process::exit(1);
    }

    let i = target.unwrap().0;
    tasks[i].description = description.to_string();
    tasks[i].updated_at = Some(Local::now());

    save_tasks(tasks);
}

pub fn mark_tasks(id: &u32, status: TaskStatus) {
    let mut tasks = load_tasks();

    let target = tasks.iter().enumerate().find(|t| &t.1.id == id);

    if let None = target {
        eprintln!("Error: Task not found: {}", id);
        std::process::exit(1);
    }

    let i = target.unwrap().0;
    tasks[i].status = status;
    tasks[i].updated_at = Some(Local::now());

    save_tasks(tasks);
}

pub fn list_tasks() {
    let tasks = load_tasks();

    let mut table = table!(["Id", "Description", "Status", "Created at", "Updated at"]);
    for task in tasks {
        table.add_row(row![
            task.id,
            task.description,
            task.status,
            task.created_at.format("%d/%m/%Y %H:%M"),
            task.updated_at
                .map(|dt| dt.format("%d/%m/%y %H:%M").to_string())
                .unwrap_or_else(|| "never".to_string())
        ]);
    }

    table.printstd();
}

pub fn clean_tasks() {
    match fs::remove_file("tasks.json") {
        Ok(_) => println!("Tasks cleaned with success"),
        Err(_) => eprintln!("Error: No tasks found"),
    }
}
