use std::{
    error::Error,
    io::{self, Write},
};

use mixe::{MIXComputer, MIXCPU};

fn main() {
    let mut input = String::new();

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
        match handle_command(command) {
            Ok(()) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn handle_command(command: &str) -> Result<(), Box<dyn Error>> {
    let computer = MIXComputer::new();
    let mut computer = MIXCPU::from(computer);

    match &command[..5] {
        "print" => {
            let content = String::from(&command[6..]);
            if content.contains('-') {
                let mid = content.find('-').unwrap();
                let left: usize = (&content[..mid]).parse().unwrap();
                let right: usize = (&content[mid..]).parse().unwrap();
                for i in &computer.computer.memory[left..=right] {
                    println!(
                        "{} {:08x} | {:030b}",
                        i.get_opposite(),
                        i.get_unsinged(),
                        i.get_unsinged()
                    );
                }
            } else if content.contains('r') {
                let regnum: usize = String::from(&command[5..])
                    .replace('A', "0")
                    .replace('X', "7")
                    .replace("J", "8")
                    .into_boxed_str()
                    .chars()
                    .filter(|x| x.is_ascii_digit())
                    .nth(0)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as usize;
                let i = computer.computer.register[regnum];
                println!(
                    "{} {:08x} | {:030b}",
                    i.get_opposite(),
                    i.get_unsinged(),
                    i.get_unsinged()
                );
            }
            Ok(())
        }
        _ => computer.run(command),
    }
}
