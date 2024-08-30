use std::error::Error;

use crate::MIXComputer;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Unit {
    id: u32,
}

impl Unit {
    pub fn new(id: u32) -> Self {
        Unit { id }
    }

    pub fn get_block_size(&self) -> Option<u32> {
        match self.id {
            0..=15 => Some(100),
            16..=17 => Some(16),
            18 => Some(24),
            19 | 20 => Some(14),
            _ => None,
        }
    }

    pub fn unit_in(&mut self, _start: usize) -> Result<(), Box<dyn Error>> {
        println!("unit number {} in", self.id);
        println!("not yet implemented.");
        Ok(())
    }

    pub fn unit_out(&self, start: usize, computer: &MIXComputer) -> Result<(), Box<dyn Error>> {
        println!("unit number {}", self.id);
        let size = self.get_block_size().ok_or("Unit invalid.")? as usize;
        for i in start..(start + size) {
            println!("[INFO] unit {} : {}", self.id, computer.memory[i].0);
        }
        Ok(())
    }
}
