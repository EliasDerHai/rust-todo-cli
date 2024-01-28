use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::iter::Map;
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

    /**
     * This function takes a mutable reference to a String and returns a Command.
     * It will remove the command from the beginning of the String.
     * If the command is not found, it will return Command::Unknown.
     */
    pub fn from_str(input: &mut String) -> Command {
        for command in Command::variants() {
            let aliases = command.get_alias();
            for alias in aliases {
                if input.starts_with(alias) {
                    input.replace_range(0..alias.len()+1, ""); // plus 1 to remove the space after the command
                    return command;
                }
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

    pub fn get_argument_options(&self) -> &'static [&'static str] {
        match self {
            Command::AddTodo => &["-label", "-desc", "-due"],
            _ => &[],
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

pub fn read_command(state: &State) -> (Command, HashMap<String, String>) {
    let mut input = String::new();
    print!("{}", state.config.cli_prefix);
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut input).expect("Failed to read line");

    if state.config.clear_screen_after_command {
        clear_screen();
    }

    let command = Command::from_str(&mut input.trim().to_lowercase());
    let command_argument_options = command.get_argument_options();

    if (command_argument_options.is_empty()) {
        return (command, HashMap::new());
    }

    /*
     TODO parse the remaining input and construct a Map from the args like:
     {
         "-label": "My Todo",
         "-desc": "My Todo Description",
         "-due": "2020-12-31",
     }
     */

    (command, HashMap::new())
}


