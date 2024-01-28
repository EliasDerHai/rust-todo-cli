use std::collections::HashMap;
use std::fmt::Debug;

use crate::command::Command;
use crate::state::State;
use crate::todo::Todo;

pub fn process_command(command: Command, args: HashMap<String, String>, state: &mut State) {
    match command {
        Command::Help => println!("Commands:\n{}", Command::get_command_palette()),
        Command::Quit => std::process::exit(0),
        Command::Clear => clear_screen(),
        Command::ToggleAutoClear => toggle_auto_clear(state),
        Command::AddTodo => add_todo(state, args),
        Command::ListTodos => list_todos(state),
        Command::Unknown => println!("Unknown command"),
    }
}

pub fn clear_screen() {
    // ANSI escape code to clear terminal screen
    println!("\x1B[2J");
}

fn toggle_auto_clear(state: &mut State) {
    state.config.clear_screen_after_command = !state.config.clear_screen_after_command;
    let on_off = if state.config.clear_screen_after_command {
        "on"
    } else {
        "off"
    };
    println!("Auto clear screen turned: {}", on_off);
}

fn add_todo(state: &mut State, args: HashMap<String, String>) {
    let default_label: String = String::from("Untitled");
    let label = args.get("-label").unwrap_or(&default_label);
    let desc = args.get("-desc");

    let todo = Todo::new(
        label.to_string(),
        desc.map(|s| s.to_string()),
    );
    state.todos.push(todo);
}

fn list_todos(state: &State) {
    println!("Todos:");
    let mut index = 0;
    for todo in &state.todos {
        index += 1;
        match todo.description {
            Some(ref desc) => println!("{}: {} - {}", index, todo.label, desc),
            None => println!("{}: {}", index, todo.label),
        }
    }
}