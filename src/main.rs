use crate::process::process_command;
use crate::read_input::read_input;

mod command;
mod process;
mod state;
mod todo;
mod read_input;
mod file_io;

fn main() {
    let mut state = state::State::new();
    loop {
        let (command, args) = read_input(&state);
        process_command(command, args, &mut state);
    }
}

