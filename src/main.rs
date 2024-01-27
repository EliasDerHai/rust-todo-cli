use std::io::stdin;

use command::Command;

mod command;

fn main() {
    loop {
        let command = read_command();
        process_command(command);
    }
}

fn read_command() -> Command {
    let mut input = String::new();
    println!("Next command:");
    let _ = stdin().read_line(&mut input);

    input = input.trim().to_lowercase();
    Command::from_str(&input)
}

fn process_command(command: Command) {
    match command {
        Command::Help => println!("Commands:\n{}", Command::get_command_palette()),
        Command::Quit => std::process::exit(0),
        Command::UNKNOWN => println!("Unknown command"),
    }
}
