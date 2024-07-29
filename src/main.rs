use std::fs::File;
use std::path::Path;
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

    if command.len() >= 7 {
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
                        .ok_or("Argument Invalid: range error")?
                        .to_digit(10)
                        .ok_or("Argument Invalid: range error")?
                        as usize;
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
            "start" => {
                computer.location = String::from(&command[6..]).parse()?;
                println!("start at location {}", computer.location);
                computer.start();
                Ok(())
            }
            "store" => {
                let location = String::from(&command[6..]);
                let data = serde_json::to_string(&computer.computer).unwrap();
                let path = Path::new(&location);
                let mut file = File::create(&path)?;
                file.write_all(data.as_bytes())?;
                Ok(())
            }
            "carry" => {
                let location = String::from(&command[6..]);
                let data = std::fs::read_to_string(&location)?;
                computer.computer = serde_json::from_str(&data)?;
                Ok(())
            }
            _ => computer.run(command),
        }
    } else {
        computer.run(command)
    }
}
