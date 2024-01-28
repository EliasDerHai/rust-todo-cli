use std::io::{stdout, Write};

use crate::command::Command;

pub fn process_command(command: Command) {
    match command {
        Command::Help => println!("Commands:\n{}", Command::get_command_palette()),
        Command::Quit => std::process::exit(0),
        Command::Clear => clear_screen(),
        Command::Unknown => println!("Unknown command"),
    }
}

const CLEAR_SCREEN_ANSI: &str = "\x1B[2J";

fn clear_screen() {
    // The ANSI escape code "\x1B[2J" clears the screen in most terminal emulators.
    print!("{}", CLEAR_SCREEN_ANSI);
    stdout().flush().expect("Failed to flush stdout");
}