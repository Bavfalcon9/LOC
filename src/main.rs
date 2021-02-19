use std::env;
mod count;
mod command;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let command: String = args.pop().unwrap();
    command::run_command(command, args);
}
