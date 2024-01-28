use crate::command::read_command;
use crate::process::process_command;

mod command;
mod process;
mod state;

fn main() {
    let mut state = state::State::new();
    loop {
        let command = read_command(&state);
        process_command(command, &mut state);
    }
}

