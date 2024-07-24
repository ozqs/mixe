use crate::mixcomputer::MIXComputer;
use crate::mixword::MIXWord;
use std::error::Error;

pub struct MIXCPU {
    location: usize,
    pub computer: MIXComputer,
}

impl MIXCPU {
    // associate functions.

    pub fn from(computer: MIXComputer) -> Self {
        MIXCPU {
            location: 0usize,
            computer,
        }
    }

    // public functions.

    /// to solve a command str mentioned in the Book.
    pub fn run(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        match self.parse(command) {
            Ok(ins) => self.execute(ins),
            Err(e) => Err(e),
        }
    }

    pub fn parse(&mut self, command: &str) -> Result<MIXWord, Box<dyn Error>> {
        let mut oprest = command.splitn(2, ' ');
        let op = oprest.next().ok_or("Invalid Argument")?;
        let rest = oprest.next().ok_or("Invalid Argument")?;
        let mut operation = MIXWord::from(0u32);
        let default_f = if op != "STJ" { 5 } else { 2 };

        self.parse_f(&mut operation, &rest, default_f)?;
        self.parse_i(&mut operation, &rest)?;
        self.parse_aa(&mut operation, &rest)?;
        self.parse_op(&mut operation, &op)?;

        Ok(operation)
    }

    pub fn execute(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        match ins.get_op() {
            8..=23 => self.execute_load(ins),
            24..=33 => self.execute_store(ins),
            1..=4 => self.execute_arithmetic(ins),
            _ => unimplemented!(),
        }
    }

    pub fn execute_in_location(&mut self) -> Result<(), Box<dyn Error>> {
        self.location += 1;
        self.execute(self.computer.memory[self.location - 1])
    }

    // private functions.

    fn execute_arithmetic(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let address = self.calculate_address(ins)
        let (left, right) = (ins.get_f() / 8, ins.get_f() % 8);
        let v = self.computer.memory[address].get_range(left, right);
        match ins.get_op() {
            1 => {
                let result = self.computer.register[0].get_value() + v.get_value();
                if result == 0 {
                    self.computer.register[0].0 &= 0b10000000000000000000000000000000;
                } else {
                    // TO DO
                    let abs_res = result.abs();
                    if abs_res > (1 << 30) - 1 {
                        overf
                    }
                }
            }
            _ => unreachable!()
        }
        Ok(())
    }

    fn calculate_address(&self, ins: MIXWord) -> usize {
        (ins.get_m()
            + if ins.get_i() != 0 {
            self.computer.register[ins.get_i() as usize].0 as i32
        } else {
            0i32
        }) as usize
    }

    fn execute_load(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        // Load Operations
        let address = self.calculate_address(ins);
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

    fn execute_store(&mut self, ins: MIXWord) -> Result<(), Box<dyn Error>> {
        let address = self.calculate_address(ins);

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

    fn parse_f(
        &self,
        operation: &mut MIXWord,
        rest: &str,
        default_f: u32,
    ) -> Result<(), Box<dyn Error>> {
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
            operation.set_f(default_f);
        }
        Ok(())
    }

    fn parse_i(&self, operation: &mut MIXWord, rest: &str) -> Result<(), Box<dyn Error>> {
        if rest.contains(',') {
            let pos = rest.find(',').unwrap();
            let i: u32 = rest[(pos + 1)..(pos + 2)].parse()?;
            operation.set_i(i);
        }
        Ok(())
    }

    fn parse_aa(&self, operation: &mut MIXWord, rest: &str) -> Result<(), Box<dyn Error>> {
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
        Ok(())
    }

    fn parse_op(&self, operation: &mut MIXWord, op: &str) -> Result<(), Box<dyn Error>> {
        match &op[..2] {
            "LD" => {
                let reg = String::from(&op[2..3]).replace('A', "0").replace('X', "7");
                let num: u32 = reg.parse()?;
                let is_negative = reg.contains('N');
                let c = num + 8 + 16 * (is_negative as u32);
                operation.set_op(c);
            }
            "ST" => {
                let reg = String::from(&op[2..3])
                    .replace('A', "0")
                    .replace('X', "7")
                    .replace('J', "8")
                    .replace('Z', "9");
                let num: u32 = reg.parse()?;
                operation.set_op(num + 24);
            }
            "AD" => operation.set_op(1),
            "SU" => operation.set_op(2),
            "MU" => operation.set_op(3),
            "DI" => operation.set_op(4),
            "EN" | "IN" | "DE" => {
                if op.len() == 2 {
                    // IN Operation
                    operation.set_op(36)
                } else {
                    let reg = String::from(&op[3..4]).replace('A', "0").replace('X', "7");
                    let reg: u32 = reg.parse()?;
                    operation.set_op(48 + reg);
                }
            }
            "CM" => {
                let reg = String::from(&op[3..4]).replace('A', "0").replace('X', "7");
                let reg: u32 = reg.parse()?;
                operation.set_op(56 + reg);
            }
            "JM" | "JS" | "JO" | "JN" | "JL" | "JG" | "JE" => operation.set_op(39),
            "JA" | "JX" | "J1" | "J2" | "J3" | "J4" | "J5" | "J6" => {
                let reg = String::from(&op[1..2]).replace('A', "0").replace('X', "7");
                let reg: u32 = reg.parse()?;
                operation.set_op(40 + reg);
            }
            "SL" | "SR" => operation.set_op(6),
            "MO" => operation.set_op(7),
            "NO" => operation.set_op(0),
            "HL" => {
                operation.set_op(5);
                operation.set_f(2);
            }
            "JB" => operation.set_op(34),
            "IO" => operation.set_op(35),
            "OU" => operation.set_op(37),
            "JR" => operation.set_op(38),
            "NU" => {
                operation.set_op(5);
                operation.set_f(0);
            }
            "CH" => {
                operation.set_op(5);
                operation.set_f(1);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }
}
