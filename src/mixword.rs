#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MIXWord(u32);

impl MIXWord {
    pub fn set_op(&mut self, c: u32) {
        self.0 = (((self.0 >> 6) << 6) + c) & 0b111111;
    }
    pub fn get_op(&self) -> u32 {
        self.0 & 0b111111
    }
    pub fn set_f(&mut self, c: u32) {
        self.0 = (self.0 & 0b11111111111111111111000000111111) + ((c & 0b111111) << 6);
    }
    pub fn get_f(&self) -> u32 {
        (self.0 >> 6) & 0b111111
    }
    pub fn set_i(&mut self, c: u32) {
        self.0 = (self.0 & 0b11111111111111000000111111111111) + ((c & 0b111111) << 12);
    }
    pub fn get_i(&self) -> u32 {
        (self.0 >> 12) & 0b111111
    }
    pub fn set_opposite(&mut self, c: u32) {
        self.0 = (self.0 & 0b01111111111111111111111111111111) + ((c & 1) << 31);
    }
    pub fn get_opposite(&self) -> u32 {
        self.0 >> 31
    }
    pub fn set_aa(&mut self, c: u32) {
        self.0 = (self.0 & 0b11000000000000111111111111111111) + ((c & 0b111111111111) << 18);
    }
    pub fn get_aa(&self) -> u32 {
        (self.0 >> 18) & 0b111111111111
    }
    pub fn get_m(&self) -> i32 {
        (self.get_aa() as i32) * (if self.get_opposite() == 1 { -1 } else { 1 })
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
