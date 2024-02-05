use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::{self, BufRead, BufReader};
use std::time::SystemTime;

use crate::state::{Config, State};
use crate::todo::Todo;

fn save_todos(state: &State) -> std::io::Result<()> {
    let file = File::create("todos.csv")?;
    let mut writer = BufWriter::new(file);

    for todo in &state.todos {
        writeln!(writer, "{},{},{}",
                 todo.label,
                 todo.description.as_ref().unwrap_or(&"".to_string()),
                 todo.completed
        )?;
    }

    Ok(())
}


fn save_settings(state: &State) -> std::io::Result<()> {
    let file = File::create("settings.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "clear_screen_after_command,{}", state.config.clear_screen_after_command)?;
    writeln!(writer, "cli_prefix,{}", state.config.cli_prefix)?;

    Ok(())
}


fn load_todos() -> io::Result<Vec<Todo>> {
    let file = File::open("todos.csv")?;
    let reader = BufReader::new(file);
    let mut todos = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 4 {
            let label = parts[0].to_string();
            let description = if parts[1].is_empty() { None } else { Some(parts[1].to_string()) };
            let due_date = parts[2].parse::<SystemTime>().ok(); // Adjust parsing based on your format
            let completed = parts[3].parse::<bool>().unwrap_or(false);

            todos.push(Todo {
                label,
                description,
                completed,
            });
        }
    }

    Ok(todos)
}


fn load_settings() -> io::Result<Config> {
    let file = File::open("settings.csv")?;
    let reader = BufReader::new(file);
    let mut config = Config::default();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        match parts[0] {
            "clear_screen_after_command" => {
                config.clear_screen_after_command = parts.get(1).and_then(|&s| s.parse().ok()).unwrap_or_default();
            },
            "cli_prefix" => {
                config.cli_prefix = parts.get(1).map(|s| s.to_string()).unwrap_or_default();
            },
            _ => {}
        }
    }

    Ok(config)
}