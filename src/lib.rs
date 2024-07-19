use std::cmp::Ordering;

// type MIXWord = i32;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MIXWord(u32);

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
    pub fn run(&mut self, command: &str) {
        let mut s = command.split(' ');
        if let(op) = s.next().unwrap() {
            
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
