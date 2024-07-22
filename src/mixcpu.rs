use crate::mixcomputer::MIXComputer;
use crate::mixword::MIXWord;
use std::error::Error;

pub struct MIXCPU {
    location: usize,
    pub computer: MIXComputer,
}

impl MIXCPU {
    pub fn from(computer: MIXComputer) -> Self {
        MIXCPU {
            location: 0usize,
            computer,
        }
    }

    pub fn excute(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        match ins.get_op() {
            8..=23 => {
                // Load Operations
                if ins.get_m() < 0 || ins.get_m() >= 4000 {
                    return Err("Index out of range.".into());
                }
                let memory_data = self.computer.memory[ins.get_m() as usize];
                let (regnum, oppo) = ((ins.get_op() - 8) % 8, (ins.get_op() - 8) / 8);
                let (left, right) = (ins.get_f() / 8, ins.get_f() % 8);
                println!(
                    "{:?},{},{},{},{},{:?}",
                    memory_data,
                    regnum,
                    oppo,
                    left,
                    right,
                    memory_data.get_range(left, right)
                );
                self.computer.register[regnum as usize] = memory_data.get_range(left, right);
                if oppo == 1 {
                    self.computer.register[regnum as usize]
                        .set_opposite(1 - self.computer.register[regnum as usize].get_opposite());
                }
                Ok(())
            }
            _ => unimplemented!(),
        }
    }

    pub fn prase(&mut self, command: &str) -> Result<MIXWord, Box<dyn Error>> {
        let mut oprest = command.splitn(2, ' ');
        let op = oprest.next().ok_or("Invalid Argument")?;
        let rest = oprest.next().ok_or("Invalid Argument")?;
        let mut operation = MIXWord::from(0u32);
        let default_f = if op != "STJ" { 5 } else { 2 };

        match &op[..2] {
            "LD" => {
                // load opers
                // get op
                let reg = String::from(&op[2..3]).replace('A', "0").replace('X', "7");
                let num: u32 = reg.parse()?;
                let is_negative = reg.contains('N');
                let c = num + 8 + 16 * (is_negative as u32);
                operation.set_op(c);

                // default_f = 5;
            }
            _ => unimplemented!(),
        }

        // get F
        if rest.contains('(') {
            let left = rest.find('(').unwrap();
            let right = rest.find(')').ok_or("Argument Invalid.")?;
            if rest.contains(':') {
                let mid = rest.find(':').unwrap();
                let left: u32 = rest[left + 1..mid].parse()?;
                let right: u32 = rest[mid + 1..right].parse()?;
                operation.set_f(left * 8 + right);
            } else {
                let val: u32 = rest[left + 1..right].parse()?;
                operation.set_f(val);
            }
        } else {
            operation.set_f(default_f); // 5 always for LD* operators
        }

        // get I
        if rest.contains(',') {
            let pos = rest.find(',').unwrap();
            let i: u32 = rest[(pos + 1)..(pos + 2)].parse()?;
            operation.set_i(i);
        }
        // else 0

        // get AA
        let mut address = 0xffffffffu32;
        for i in rest.chars() {
            if i.is_ascii_digit() {
                if address == 0xffffffffu32 {
                    address = i.to_digit(10).unwrap();
                } else {
                    address = address * 10 + i.to_digit(10).unwrap();
                }
            } else if address != 0xffffffffu32 {
                break;
            }
        }
        operation.set_aa(address);

        if rest.contains('-') {
            operation.set_opposite(1);
        }

        Ok(operation)
    }

    /// to solve a command str mentioned in the Book.
    pub fn run(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        match self.prase(command) {
            Ok(ins) => self.excute(ins),
            Err(e) => Err(e),
        }
    }
}
