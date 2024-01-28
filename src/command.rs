use std::io::{stdin, stdout, Write};

pub enum Command {
    Help,
    Quit,
    Clear,
    Unknown,
}

impl Command {
    pub fn variants() -> Vec<Command> {
        vec![
            Command::Help,
            Command::Quit,
            Command::Clear,
        ]
    }

    pub fn from_str(input: &str) -> Command {
        for cmd in Command::variants() {
            if cmd.get_alias().contains(&input) {
                return cmd;
            }
        }
        Command::Unknown
    }

    pub fn get_alias(&self) -> &'static [&'static str] {
        match self {
            Command::Help => &["help", "h", "/h"],
            Command::Quit => &["quit", "q", "/q"],
            Command::Clear => &["clear", "c", "/c"],
            Command::Unknown => &[],
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Command::Help => "Show this help message",
            Command::Quit => "Quit the program",
            Command::Clear => "Clear console",
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

pub fn read_command() -> Command {
    let mut input = String::new();
    print!("$");
    stdout().flush().expect("Failed to flush stdout");
    let _ = stdin().read_line(&mut input);

    input = input.trim().to_lowercase();
    Command::from_str(&input)
}


