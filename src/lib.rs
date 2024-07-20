use std::{cmp::Ordering, error::Error};

// type MIXWord = i32;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MIXWord(u32);

impl MIXWord {
    pub fn set_op(&mut self, c: u32) {
        // self.0 &= 0b11111111111111111111111111100000;
        // self.0 &= c | 0b11111111111111111111111111100000;
        self.0 = ((self.0 >> 5) << 5) + c & 0b11111;
    }
    pub fn set_f(&mut self, c: u32) {
        self.0 = self.0 & 0b11111111111111111111110000011111 + (c & 0b11111) << 5;
    }
}

impl From<u32> for MIXWord {
    fn from(a: u32) -> Self {
        MIXWord(a)
    }
}

impl From<(u32, u32, u32, u32, u32, u32)> for MIXWord {
    fn from(a: (u32, u32, u32, u32, u32, u32)) -> Self {
        MIXWord((a.0 << 31) + (a.1 << 20) + (a.2 << 15) + (a.3 << 10) + (a.4 << 5) + (a.5))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Unit {}

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

    pub fn excute(&mut self, ins: i32) {
        todo!()
    }

    // to solve a command str mentioned in the Book.
    pub fn run(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        let mut s = command.split(' ');
        let op = s.next().ok_or("Invalid Argument")?;
        let mut operation = MIXWord::from(0);

        match &op[..2] {
            "LD" => {
                // load opers
                let reg = String::from(&op[2..3]).replace("A", "0").replace("X", "7");
                let num: u32 = reg.parse()?;
                let is_negative = reg.contains("N");
                operation.set_op(num + 8 + 16 * (is_negative as u32));
                // match &op[3..] {

                //     _ => return Err("Argument Invalid.".into())
                // }
                
                if op.contains("(") {
                    let left = op.find('(').unwrap();
                    let right = op.find(')').ok_or("Argument Invalid.")?;
                    if op.contains(";") {
                        let mid = op.find(':').unwrap();
                        let left: u32 = (&op[left+1..mid]).parse()?;
                        let right: u32 = (&op[mid+1..right]).parse()?;
                        operation.set_f(left * 8 + right);
                    }
                    else {
                        let val: u32 = op[left+1..right].parse()?;
                        operation.set_f(val);
                    }
                }

                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load() {
        let mut computer = MIXComputer::new();
        // computer.memory[2000] = ((1 << 31) + (80 << 15) + (3 << 10) + (5 << 5) + 4).into();
        computer.memory[2000] = (1, 0, 80, 3, 5, 4).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("LDA 2000");
        assert_eq!(computer.computer.register[0], (1, 0, 80, 3, 5, 4).into());
        computer.run("LDA 2000(1:5)");
        assert_eq!(computer.computer.register[0], (0, 0, 80, 3, 5, 4).into());
        computer.run("LDA 2000(3:5)");
        assert_eq!(computer.computer.register[0], (0, 0, 0, 3, 5, 4).into());
        computer.run("LDA 2000(0:3)");
        assert_eq!(computer.computer.register[0], (1, 0, 0, 0, 80, 3).into());
        computer.run("LDA 2000(4:4)");
        assert_eq!(computer.computer.register[0], 5.into());
        computer.run("LDA 2000(0:0)");
        assert_eq!(computer.computer.register[0], (1, 0, 0, 0, 0, 0).into());
    }
}
