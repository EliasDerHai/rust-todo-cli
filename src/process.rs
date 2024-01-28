use crate::command::Command;
use crate::state::State;
use crate::todo::Todo;

pub fn process_command(command: Command, state: &mut State) {
    match command {
        Command::Help => println!("Commands:\n{}", Command::get_command_palette()),
        Command::Quit => std::process::exit(0),
        Command::Clear => clear_screen(),
        Command::ToggleAutoClear => toggle_auto_clear(state),
        Command::AddTodo => add_todo(state),
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

fn add_todo(state: &mut State) {
    let todo = Todo::new(String::from("New Todo"));
    state.todos.push(todo);
}

fn list_todos(state: &State) {
    println!("Todos:");
    let mut index = 0;
    for todo in &state.todos {
        index += 1;
        println!("{}: {}", index, todo.label);
    }
}