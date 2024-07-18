use std::cmp::Ordering;
type word = i32;

struct Unit {}

impl Unit {
    fn new() -> Self {Unit}
}

struct MIXComputer {
    register: [word; 9],
    overflow: bool,
    comp: Ordering, // -1 0 1
    units: [Unit; 16],
    memory: [word; 4000],
}

impl MIXComputer {
    fn new() -> Self {
        MIXComputer {
            register: [0; 9],
            overflow: false,
            comp: Ordering::Less,
            units: [Unit::new(); 16],
            memory[0; 4000],
        }
    }
}

struct MIXCPU {
    location: usize,
    computer: MIXComputer,
}

impl MIXCPU {
    fn from(computer: MIXComputer) -> Self {
        MIXCPU {
            location: 0usize,
            computer,
        }
    }
    
    fn excute(ins: i32) {
        todo!()
    }

    fn run(command: &str) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_load() {
        let computer = MIXComputer::new();
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