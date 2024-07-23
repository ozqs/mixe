mod mixcomputer;
mod mixcpu;
mod mixword;
mod unit;

pub use std::error::Error;

// use mixword::MIXWord;
pub use mixcomputer::MIXComputer;
pub use mixcpu::MIXCPU;
pub use mixword::MIXWord;
pub use unit::Unit;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser() {
        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 0, 80, 3, 5, 4).into();
        let mut computer = MIXCPU::from(computer);
        assert_eq!(
            computer.parse("LDA 2000,2(0:3)").unwrap(),
            (0, 0, 2000, 2, 3, 8).into()
        );
        assert_eq!(
            computer.parse("LDA 2000,2(1:3)").unwrap(),
            (0, 0, 2000, 2, 11, 8).into()
        );
        assert_eq!(
            computer.parse("LDA 2000(1:3)").unwrap(),
            (0, 0, 2000, 0, 11, 8).into()
        );
        assert_eq!(
            computer.parse("LDA 2000").unwrap(),
            (0, 0, 2000, 0, 5, 8).into()
        );
        assert_eq!(
            computer.parse("LDA -2000,4").unwrap(),
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

    #[test]
    fn test_store() {
        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 1, 2, 3, 4, 5).into();
        computer.register[0] = (0, 6, 7, 8, 9, 0).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("STA 2000").unwrap();
        // 00_000000_001001_001000_000111_000110
        // 0  0      9      8      7      6      
        // 00_000110_000111_001000_001001_000000
        assert_eq!(computer.computer.memory[2000], (0, 6, 7, 8, 9, 0).into());

        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 1, 2, 3, 4, 5).into();
        computer.register[0] = (0, 6, 7, 8, 9, 0).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("STA 2000(1:5)").unwrap();
        assert_eq!(computer.computer.memory[2000], (1, 6, 7, 8, 9, 0).into());

        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 1, 2, 3, 4, 5).into();
        computer.register[0] = (0, 6, 7, 8, 9, 0).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("STA 2000(5:5)").unwrap();
        assert_eq!(computer.computer.memory[2000], (1, 1, 2, 3, 4, 0).into());

        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 1, 2, 3, 4, 5).into();
        computer.register[0] = (0, 6, 7, 8, 9, 0).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("STA 2000(2:2)").unwrap();
        assert_eq!(computer.computer.memory[2000], (1, 1, 0, 3, 4, 5).into());

        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 1, 2, 3, 4, 5).into();
        computer.register[0] = (0, 6, 7, 8, 9, 0).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("STA 2000(2:3)").unwrap();
        assert_eq!(computer.computer.memory[2000], (1, 1, 9, 0, 4, 5).into());

        let mut computer = MIXComputer::new();
        computer.memory[2000] = (1, 1, 2, 3, 4, 5).into();
        computer.register[0] = (0, 6, 7, 8, 9, 0).into();
        let mut computer = MIXCPU::from(computer);
        computer.run("STA 2000(0:1)").unwrap();
        assert_eq!(computer.computer.memory[2000], (0, 0, 2, 3, 4, 5).into());
    }
}
