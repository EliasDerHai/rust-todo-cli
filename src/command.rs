use std::io::{stdin, stdout, Write};
use crate::process::clear_screen;
use crate::state::State;

pub enum Command {
    Help,
    Quit,
    Clear,
    ToggleAutoClear,
    AddTodo,
    ListTodos,
    Unknown,
}

impl Command {
    /**
     * This function returns a vector of all the variants (except Unknown) of the Command enum.
     * newly added Commands have to be added to the vector!
     */
    pub fn variants() -> Vec<Command> {
        vec![
            Command::Help,
            Command::Quit,
            Command::Clear,
            Command::ToggleAutoClear,
            Command::AddTodo,
            Command::ListTodos,
        ]
    }

    pub fn from_str(input: &str) -> Command {
        for command in Command::variants() {
            if command.get_alias().contains(&input) {
                return command;
            }
        }
        Command::Unknown
    }

    pub fn get_alias(&self) -> &'static [&'static str] {
        match self {
            Command::Help => &["help", "h", "/h"],
            Command::Quit => &["quit", "q", "/q"],
            Command::Clear => &["clear", "c", "/c"],
            Command::ToggleAutoClear => &["autoclear", "ac", "/ac"],
            Command::AddTodo => &["add", "a", "/a"],
            Command::ListTodos => &["list", "l", "/l"],
            Command::Unknown => &[],
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Command::Help => "Show this help message",
            Command::Quit => "Quit the program",
            Command::Clear => "Clear console",
            Command::ToggleAutoClear => "Toggles auto clear on/off",
            Command::AddTodo => "Add a new todo",
            Command::ListTodos => "List all todos",
            Command::Unknown => "",
        }
    }

    pub fn get_command_palette() -> String {
        Command::variants()
            .iter()
            .map(|cmd| {
                let aliases = cmd.get_alias();
                let description = cmd.get_description();
                format!("\"{}\" - {}", aliases.join("\", \""), description)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn read_command(state: &State) -> Command {
    let mut input = String::new();
    print!("{}", state.config.cli_prefix);
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut input).expect("Failed to read line");

    if state.config.clear_screen_after_command {
        clear_screen();
    }

    Command::from_str(&input.trim().to_lowercase())
}


