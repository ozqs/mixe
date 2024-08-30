use std::io::{self, Write};
mod command_handler;
use command_handler::handle_command;
use mixe::{MIXComputer, MIXCPU};

fn main() {
    let mut input = String::new();
    let computer = MIXComputer::new();
    let mut computer = MIXCPU::from(computer);

    loop {
        // Display prompt
        print!(">> ");
        io::stdout().flush().unwrap();

        // Read user input
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim whitespace
        let command = input.trim();

        // Check for exit condition
        if command.eq_ignore_ascii_case("exit") || command.eq_ignore_ascii_case("quit") {
            println!("Goodbye!");
            break;
        }

        // Handle the command
        match handle_command(command, &mut computer) {
            Ok(()) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

