use std::time::Instant;

pub struct Todo {
    pub label: String,
    pub description: Option<String>,
    pub due_date: Option<Instant>,
    pub completed: bool,
}

impl Todo {
    pub fn new(label: String) -> Todo {
        Todo {
            label,
            description: None,
            due_date: None,
            completed: false,
        }
    }
}