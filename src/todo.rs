pub struct Todo {
    pub label: String,
    pub description: Option<String>,
    pub completed: bool,
}

impl Todo {
    pub fn new(
        label: String,
        description: Option<String>,
    ) -> Todo {
        Todo {
            label,
            description,
            completed: false,
        }
    }
}