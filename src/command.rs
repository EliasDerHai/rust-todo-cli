
pub enum Command {
    Help,
    Quit,
    UNKNOWN,
}

impl Command {
    pub fn variants() -> Vec<Command> {
        vec![
            Command::Help,
            Command::Quit,
        ]
    }

    pub fn from_str(input: &str) -> Command {
        match input.trim().to_lowercase().as_str() {
            "help" | "h" | "/h" => Command::Help,
            "quit" | "q" | "/q" => Command::Quit,
            _ => Command::UNKNOWN
        }
    }

    pub fn get_alias(&self) -> &'static [&'static str] {
        match self {
            Command::Help => &["help", "h", "/h"],
            Command::Quit => &["quit", "q", "/q"],
            Command::UNKNOWN => &[],
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            Command::Help => "Show this help message",
            Command::Quit => "Quit the program",
            Command::UNKNOWN => "",
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

