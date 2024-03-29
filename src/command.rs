
pub enum Command {
    Help,
    Quit,
    Clear,
    ToggleAutoClear,
    AddTodo,
    ListTodos,
    SaveToFile,
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
            Command::SaveToFile,
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
                    input.replace_range(0..alias.len(), "");
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
            Command::SaveToFile => &["save", "s", "/s"],
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
            Command::SaveToFile => "Save to file [Todos & Config]",
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
