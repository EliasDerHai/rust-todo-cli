use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

use crate::command::Command;
use crate::process::clear_screen;
use crate::state::State;

pub fn read_input(state: &State) -> (Command, HashMap<String, String>) {
    let mut input = String::new();
    print!("{}", state.config.cli_prefix);
    stdout().flush().expect("Failed to flush stdout");
    stdin().read_line(&mut input).expect("Failed to read line");

    if state.config.clear_screen_after_command {
        clear_screen();
    }

    input = input.trim().to_lowercase();
    let command = Command::from_str(&mut input);
    let command_argument_options = command.get_argument_options();

    if command_argument_options.is_empty() {
        return (command, HashMap::new());
    }

    let mut args_map = HashMap::new();
    extract_arguments(&input, command_argument_options, &mut args_map);
    (command, args_map)
}

fn extract_arguments(
    input: &str,
    command_argument_options: &[&str],
    map: &mut HashMap<String, String>,
) {
    let mut input = input.trim();
    while !input.is_empty() {
        let first_match = command_argument_options
            .iter()
            .find(|&&argument_option| input.starts_with(argument_option));

        match first_match {
            Some(argument) => {
                let remainder = &input[argument.len()..].trim_start();
                let (argument_value, rest) = extract_value(remainder);
                map.insert(argument.to_string(), argument_value.to_string());
                input = rest;
            }
            None => {
                let next_space_or_end = input.find(' ').unwrap_or(input.len());
                input = &input[next_space_or_end..];
            }
        }
    }
}

fn extract_value(input: &str) -> (&str, &str) {
    if input.starts_with('\"') {
        let end_quote = input[1..].find('\"').unwrap_or_else(|| input.len() - 1);
        let value = &input[1..=end_quote];
        let rest = &input[end_quote + 2..]; // Skip closing quote and space
        (value, rest)
    } else {
        let end = input.find(' ').unwrap_or(input.len());
        let value = &input[..end];
        let rest = &input[end..].trim_start();
        (value, rest)
    }
}