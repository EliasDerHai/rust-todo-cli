use crate::command::read_command;
use crate::process::process_command;

mod command;
mod process;

fn main() {
    loop {
        let command = read_command();
        process_command(command);
    }
}

