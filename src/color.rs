#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rgb(pub u8, pub u8, pub u8);

pub fn color_grayscale(value: u8) -> Rgb {
    Rgb(value, value, value)
}
