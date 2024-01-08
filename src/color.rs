#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Rgb {
    pub fn as_u32(self) -> u32 {
        u32::from_be_bytes([0, self.0, self.1, self.2])
    }
}

pub fn grayscale(value: u8) -> Rgb {
    Rgb(value, value, value)
}
