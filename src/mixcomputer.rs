use crate::mixword::MIXWord;
use crate::Unit;
use std::cmp::Ordering;

pub struct MIXComputer {
    // A J1 J2 J3 J4 J5 J6 X J
    pub register: [MIXWord; 9],
    pub overflow: bool,
    pub comp: Ordering, // -1 0 1
    pub units: [Unit; 16],
    pub memory: [MIXWord; 4000],
}

impl Default for MIXComputer {
    fn default() -> Self {
        Self::new()
    }
}

impl MIXComputer {
    pub fn new() -> Self {
        MIXComputer {
            register: [0u32.into(); 9],
            overflow: false,
            comp: Ordering::Less,
            units: std::array::from_fn(|i| Unit::new(i as u32)),
            memory: [0u32.into(); 4000],
        }
    }
}
