use crate::todo::Todo;

pub struct State {
    pub todos: Vec<Todo>,
    pub config: Config,
}

impl State {
    pub fn new() -> State {
        State {
            todos: vec![],
            config: Config::default(),
        }
    }
}

pub struct Config {
    pub clear_screen_after_command: bool,
    pub cli_prefix: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            clear_screen_after_command: true,
            cli_prefix: String::from("$"),
        }
    }
}