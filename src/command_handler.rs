use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use mixe::{MIXCPU, MIXWord};

pub fn handle_command(command: &str, computer: &mut MIXCPU) -> Result<(), Box<dyn Error>> {

    if command.len() >= 7 {
        match &command[..5] {
            "print" => {
                let content = String::from(&command[6..]);
                if content.contains('-') {
                    let mid = content.find('-').unwrap();
                    let left: usize = content[..mid].parse().unwrap();
                    let right: usize = content[mid+1..].parse().unwrap();
                    if !mixe::mixcpu::MEMORY_RANGE.contains(&right) {
                        return Err("Index out of range".into());
                    }
                    if !mixe::mixcpu::MEMORY_RANGE.contains(&left) {
                        return Err("Index out of range".into());
                    }
                    for i in &computer.computer.memory[left..=right] {
                        println!(
                            "({}) {} {:08x} | {:030b}",
                            i,
                            i.get_opposite(),
                            i.get_unsinged(),
                            i.get_unsinged()
                        );
                    }
                } else if content.contains('r') {
                    let regnum: usize = String::from(&command[5..])
                        .replace('A', "0")
                        .replace('X', "7")
                        .replace('J', "8")
                        .into_boxed_str()
                        .chars().find(|x| x.is_ascii_digit())
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
                if let Ok(x) = String::from(&command[6..]).parse() {
                    computer.location = x
                }

                println!("start at location {}", computer.location);

                computer.start();
                Ok(())
            }
            "store" => {
                let location = String::from(&command[6..]);
                let data = serde_json::to_string(&computer.computer).unwrap();
                let path = Path::new(&location);
                let mut file = File::create(path)?;
                file.write_all(data.as_bytes())?;
                Ok(())
            }
            "carry" => {
                let location = String::from(&command[6..]);
                let data = std::fs::read_to_string(location)?;
                computer.computer = serde_json::from_str(&data)?;
                Ok(())
            }
            "parse" => {
                let location = String::from(&command[6..]);
                for i in std::fs::read_to_string(location)?.lines() {
                    let mut foo = i.splitn(2, ' ');
                    let left = foo.next().ok_or("")?;
                    let right = foo.next().ok_or("")?;
                    let left: usize = left.parse()?;
                    let right_parsed: MIXWord = right.try_into()?;
                    println!("Set memory {} to {} : {}", left, right, right_parsed);
                    computer.computer.memory[left] = right_parsed;
                }
                Ok(())
            }
            _ => computer.run_command(command),
        }
    } else {
        computer.run_command(command)
    }
}