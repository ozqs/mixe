use std::cmp::Ordering;
type word = i32;

#[derive(Clone, Copy)]
struct Unit {}

impl Unit {
    fn new() -> Self {Unit{} }
}

pub struct MIXComputer {
    pub register: [word; 9],
    pub overflow: bool,
    pub comp: Ordering, // -1 0 1
    pub units: [Unit; 16],
    pub memory: [word; 4000],
}

impl MIXComputer {
    fn new() -> Self {
        MIXComputer {
            register: [0; 9],
            overflow: false,
            comp: Ordering::Less,
            units: [Unit::new(); 16],
            memory: [0; 4000],
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
    
    pub fn excute(ins: i32) {
        todo!()
    }

    pub fn run(command: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load() {
        let mut computer = MIXComputer::new();
        computer.memory[2000] = -((80 << 15) + (3 << 10) + (5 << 5) + 4);
        let computer = MIXCPU::from(computer);
        computer.run("LDA 2000");
        assert_eq!(computer.computer.register[0], -((80 << 15) + (3 << 10) + (5 << 5) + 4));
        computer.run("LDA 2000(1:5)");
        assert_eq!(computer.computer.register[0], ((80 << 15) + (3 << 10) + (5 << 5) + 4));
        computer.run("LDA 2000(3:5)");
        assert_eq!(computer.computer.register[0], ((3 << 10) + (5 << 5) + 4));
        computer.run("LDA 2000(0:3)");
        assert_eq!(computer.computer.register[0], -((80 << 5) + 3));
        computer.run("LDA 2000(4:4)");
        assert_eq!(computer.computer.register[0], 5);
        computer.run("LDA 2000(0:0)");
        assert_eq!(computer.computer.register[0], -0);
    }
}