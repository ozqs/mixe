#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {}

impl Default for Unit {
    fn default() -> Self {
        Self::new()
    }
}

impl Unit {
    pub fn new() -> Self {
        Unit {}
    }
}
