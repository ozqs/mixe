use crate::mixword::MIXWord;
use crate::Unit;
use std::cmp::Ordering;

pub struct MIXComputer {
    pub register: [MIXWord; 9],
    pub overflow: bool,
    pub comp: Ordering, // -1 0 1
    pub units: [Unit; 16],
    pub memory: [MIXWord; 4000],
}

impl MIXComputer {
    pub fn new() -> Self {
        MIXComputer {
            register: [0u32.into(); 9],
            overflow: false,
            comp: Ordering::Less,
            units: [Unit::new(); 16],
            memory: [0u32.into(); 4000],
        }
    }
}
