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

    pub fn execute_in_location(&mut self) -> Result<(), Box<dyn Error>> {
        self.location += 1;
        self.execute(self.computer.memory[self.location - 1])
    }

    pub fn execute(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        match ins.get_op() {
            8..=23 => {
                // Load Operations
                let address = ins.get_m()
                    + if ins.get_i() != 0 {
                        self.computer.register[ins.get_i() as usize].0 as i32
                    } else {
                        0i32
                    };
                if address < 0 || address >= 4000 {
                    return Err("Index out of range.".into());
                }
                let memory_data = self.computer.memory[address as usize];
                let (regnum, oppo) = ((ins.get_op() - 8) % 8, (ins.get_op() - 8) / 8);
                let (left, right) = (ins.get_f() / 8, ins.get_f() % 8);

                self.computer.register[regnum as usize] = memory_data.get_range(left, right);
                if oppo == 1 {
                    self.computer.register[regnum as usize]
                        .set_opposite(1 - self.computer.register[regnum as usize].get_opposite());
                }
                Ok(())
            }
            24..=33 => {
                let address = ins.get_m()
                    + if ins.get_i() != 0 {
                        self.computer.register[ins.get_i() as usize].0 as i32
                    } else {
                        0i32
                    };

                if address < 0 || address >= 4000 {
                    return Err("Index out of range.".into());
                }

                let memory_data = self.computer.memory[address as usize];
                let reg_data = if ins.get_op() == 33 {
                    0.into()
                } else {
                    self.computer.register[(ins.get_op() - 24) as usize]
                };

                let (mut left, right) = (ins.get_f() / 8, ins.get_f() % 8);

                let reg: Vec<u32> = reg_data.into();
                let mut mem: Vec<u32> = memory_data.into();
                if left == 0 {
                    mem[0] = reg[0];
                    left += 1;
                }
                let mut reg = reg.into_iter().rev();
                for i in (left..=right).rev() {
                    mem[i as usize] = reg.next().unwrap();
                }

                self.computer.memory[address as usize] =
                    (mem[0], mem[1], mem[2], mem[3], mem[4], mem[5]).into();

                Ok(())
            }
            _ => unimplemented!(),
        }
    }

    pub fn parse(&mut self, command: &str) -> Result<MIXWord, Box<dyn Error>> {
        let mut oprest = command.splitn(2, ' ');
        let op = oprest.next().ok_or("Invalid Argument")?;
        let rest = oprest.next().ok_or("Invalid Argument")?;
        let mut operation = MIXWord::from(0u32);
        let default_f = if op != "STJ" { 5 } else { 2 };

        match &op[..2] {
            "LD" => {
                // load operations
                // get op
                let reg = String::from(&op[2..3]).replace('A', "0").replace('X', "7");
                let num: u32 = reg.parse()?;
                let is_negative = reg.contains('N');
                let c = num + 8 + 16 * (is_negative as u32);
                operation.set_op(c);

                // default_f = 5;
            }
            "ST" => {
                // store operations
                let reg = String::from(&op[2..3])
                    .replace('A', "0")
                    .replace('X', "7")
                    .replace('J', "8")
                    .replace('Z', "9");

                let num: u32 = reg.parse()?;
                operation.set_op(num + 24);
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
        match self.parse(command) {
            Ok(ins) => self.execute(ins),
            Err(e) => Err(e),
        }
    }
}
