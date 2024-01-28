use crate::command::Command;
use crate::state::State;

pub fn process_command(command: Command, state: &mut State) {
    match command {
        Command::Help => println!("Commands:\n{}", Command::get_command_palette()),
        Command::Quit => std::process::exit(0),
        Command::Clear => clear_screen(),
        Command::ToggleAutoClear => toggle_auto_clear(state),
        Command::Unknown => println!("Unknown command"),
    }
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

pub fn clear_screen() {
    // ANSI escape code to clear terminal screen
    println!("\x1B[2J");
}