use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct MIXWord(pub u32);
pub const MASK: u32 = 0b01111111111111111111111111111111;

fn max(l: u32, r: u32) -> u32 {
    if l > r {
        l
    } else {
        r
    }
}

impl MIXWord {
    /// ### Set op
    /// set the last six bytes as same as c.
    /// ```rust
    /// use mixe::MIXWord;
    /// let mut g: MIXWord = 0b11111000000u32.into();
    /// g.set_op(0b101010);
    /// assert_eq!(g, 0b11111101010u32.into());
    /// ```
    pub fn set_op(&mut self, c: u32) {
        self.0 = ((self.0 >> 6) << 6) + (c & 0b111111);
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
        self.0 = (self.0 & MASK) + ((c & 1) << 31);
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
    /// ## with m
    pub fn get_m(&self) -> i32 {
        (self.get_aa() as i32) * (if self.get_opposite() == 1 { -1 } else { 1 })
    }
    pub fn get_range(&self, l: u32, r: u32) -> MIXWord {
        let ll = max(l, 1u32);
        let absolute_part = if r > 0 {
            (self.0 >> ((5 - r) * 6)) & ((1 << ((r - ll + 1) * 6)) - 1)
        } else {
            0
        };

        if l > 0 {
            absolute_part.into()
        } else {
            let mut ret: MIXWord = absolute_part.into();
            ret.set_opposite(self.get_opposite());
            ret
        }
    }
    pub fn set_unsigned(&mut self, c: u32) {
        let o = self.get_opposite();
        self.0 = c & MASK;
        self.set_opposite(o);
    }
    pub fn get_unsinged(&self) -> u64 {
        (self.0 & MASK) as u64
    }
    pub fn get_value(&self) -> i64 {
        (self.get_unsinged() as i64)
            * if self.get_opposite() == 1 {
                -1i64
            } else {
                1i64
            }
    }
    pub fn into_slice(self) -> (u32, u32, u32, u32, u32, u32) {
        self.into()
    }
    pub fn from_value(c: i64) -> Self {
        let mut ret = Self(c.unsigned_abs() as u32);
        ret.set_opposite(if c > 0 { 0 } else { 1 });
        ret
    }
}

impl From<u32> for MIXWord {
    fn from(a: u32) -> Self {
        MIXWord(a)
    }
}

/// ### into slices.
/// ```rust
/// use mixe::MIXWord;
/// let a = (1, 2, 3, 4, 5, 6);
/// let b: MIXWord = a.into();
/// assert_eq!(a, b.into());
/// ```
impl From<MIXWord> for (u32, u32, u32, u32, u32, u32) {
    fn from(val: MIXWord) -> Self {
        (
            val.get_opposite(),
            (val.0 >> 24) & 0b111111,
            (val.0 >> 18) & 0b111111,
            val.get_i(),
            val.get_f(),
            val.get_op(),
        )
    }
}

impl From<MIXWord> for Vec<u32> {
    fn from(val: MIXWord) -> Self {
        vec![
            val.get_opposite(),
            (val.0 >> 24) & 0b111111,
            (val.0 >> 18) & 0b111111,
            val.get_i(),
            val.get_f(),
            val.get_op(),
        ]
    }
}

impl From<MIXWord> for [u32; 6] {
    fn from(val: MIXWord) -> Self {
        [
            val.get_opposite(),
            (val.0 >> 24) & 0b111111,
            (val.0 >> 18) & 0b111111,
            val.get_i(),
            val.get_f(),
            val.get_op(),
        ]
    }
}

impl From<(u32, u32, u32, u32, u32, u32)> for MIXWord {
    fn from(a: (u32, u32, u32, u32, u32, u32)) -> Self {
        MIXWord((a.0 << 31) + (a.1 << 24) + (a.2 << 18) + (a.3 << 12) + (a.4 << 6) + (a.5))
    }
}

impl From<[u32; 6]> for MIXWord {
    fn from(a: [u32; 6]) -> Self {
        MIXWord((a[0] << 31) + (a[1] << 24) + (a[2] << 18) + (a[3] << 12) + (a[4] << 6) + (a[5]))
    }
}

impl From<Vec<u32>> for MIXWord {
    fn from(value: Vec<u32>) -> Self {
        (value[0], value[1], value[2], value[3], value[4], value[5]).into()
    }
}
