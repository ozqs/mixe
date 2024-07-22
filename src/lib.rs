use log::debug;
use std::{cmp::Ordering, error::Error};

// type MIXWord = i32;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MIXWord(u32);

impl MIXWord {
    pub fn set_op(&mut self, c: u32) {
        self.0 = ((self.0 >> 5) << 5) + c & 0b11111;
    }
    pub fn set_f(&mut self, c: u32) {
        self.0 = (self.0 & 0b11111111111111111111000000111111) + ((c & 0b111111) << 6);
    }
    pub fn set_i(&mut self, c: u32) {
        self.0 = (self.0 & 0b11111111111111000000111111111111) + ((c & 0b111111) << 12);
    }
    pub fn set_opposite(&mut self, c: u32) {
        self.0 = (self.0 & 0b01111111111111111111111111111111) + ((c & 1) << 31);
    }
    pub fn set_aa(&mut self, c: u32) {
        self.0 = (self.0 & 0b11000000000000111111111111111111) + ((c & 0b111111111111) << 18);
    }
}

impl From<u32> for MIXWord {
    fn from(a: u32) -> Self {
        MIXWord(a)
    }
}

impl From<(u32, u32, u32, u32, u32, u32)> for MIXWord {
    fn from(a: (u32, u32, u32, u32, u32, u32)) -> Self {
        MIXWord((a.0 << 31) + (a.1 << 24) + (a.2 << 18) + (a.3 << 12) + (a.4 << 6) + (a.5))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {}

impl Unit {
    fn new() -> Self {
        Unit {}
    }
}

pub struct MIXComputer {
    pub register: [MIXWord; 9],
    pub overflow: bool,
    pub comp: Ordering, // -1 0 1
    pub units: [Unit; 16],
    pub memory: [MIXWord; 4000],
}

impl MIXComputer {
    fn new() -> Self {
        MIXComputer {
            register: [0u32.into(); 9],
            overflow: false,
            comp: Ordering::Less,
            units: [Unit::new(); 16],
            memory: [0u32.into(); 4000],
        }
    }
}

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
        todo!()
    }

    pub fn prase(&mut self, command: &str) -> Result<MIXWord, Box<dyn Error>> {
        let mut oprest = command.splitn(2, " ");
        let op = oprest.next().ok_or("Invalid Argument")?;
        let rest = oprest.next().ok_or("Invalid Argument")?;
        let mut operation = MIXWord::from(0);
        let default_f = if op != "STJ" { 5 } else { 2 };

        match &op[..2] {
            "LD" => {
                // load opers
                // get op
                let reg = String::from(&op[2..3]).replace("A", "0").replace("X", "7");
                let num: u32 = reg.parse()?;
                let is_negative = reg.contains("N");
                let c = num + 8 + 16 * (is_negative as u32);
                operation.set_op(c);

                // default_f = 5;
            }
            _ => unimplemented!(),
        }

        // get F
        if rest.contains("(") {
            let left = rest.find('(').unwrap();
            let right = rest.find(')').ok_or("Argument Invalid.")?;
            if rest.contains(":") {
                let mid = rest.find(':').unwrap();
                let left: u32 = (&rest[left + 1..mid]).parse()?;
                let right: u32 = (&rest[mid + 1..right]).parse()?;
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
            if i.is_digit(10) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_praser() {
        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 0, 80, 3, 5, 4).into();
        let mut computer = MIXCPU::from(computer);
        assert_eq!(
            computer.prase("LDA 2000,2(0:3)").unwrap(),
            (0, 0, 2000, 2, 3, 8).into()
        );
        assert_eq!(
            computer.prase("LDA 2000,2(1:3)").unwrap(),
            (0, 0, 2000, 2, 11, 8).into()
        );
        assert_eq!(
            computer.prase("LDA 2000(1:3)").unwrap(),
            (0, 0, 2000, 0, 11, 8).into()
        );
        assert_eq!(
            computer.prase("LDA 2000").unwrap(),
            (0, 0, 2000, 0, 5, 8).into()
        );
        assert_eq!(
            computer.prase("LDA -2000,4").unwrap(),
            (1, 0, 2000, 4, 5, 8).into()
        );
    }
    #[test]
    fn test_load() {
        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 0, 80, 3, 5, 4).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("LDA 2000").unwrap();
        assert_eq!(computer.computer.register[0], (1, 0, 80, 3, 5, 4).into());
        computer.run("LDA 2000(1:5)").unwrap();
        assert_eq!(computer.computer.register[0], (0, 0, 80, 3, 5, 4).into());
        computer.run("LDA 2000(3:5)").unwrap();
        assert_eq!(computer.computer.register[0], (0, 0, 0, 3, 5, 4).into());
        computer.run("LDA 2000(0:3)").unwrap();
        assert_eq!(computer.computer.register[0], (1, 0, 0, 0, 80, 3).into());
        computer.run("LDA 2000(4:4)").unwrap();
        assert_eq!(computer.computer.register[0], 5.into());
        computer.run("LDA 2000(0:0)").unwrap();
        assert_eq!(computer.computer.register[0], (1, 0, 0, 0, 0, 0).into());
    }
}
