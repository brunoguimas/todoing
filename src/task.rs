use std::fmt;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Local>,
    pub updated_at: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Todo,
            created_at: Local::now(),
            updated_at: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "todo"),
            TaskStatus::Doing => write!(f, "doing"),
            TaskStatus::Done => write!(f, "done"),
        }
    }
}
