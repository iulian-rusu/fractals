#![allow(dead_code)]

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

#[derive(Debug)]
pub struct Palette {
    colors: [Rgb; Self::MAX_COLORS],
}

impl Palette {
    const MAX_COLORS: usize = 256;

    pub fn from_gradient(gradient: &[Rgb]) -> Self {
        assert!(
            gradient.len() >= 2 && gradient.len() <= Self::MAX_COLORS,
            "Color gradient must specify between 2 and {} colors",
            Self::MAX_COLORS
        );

        let range_count = gradient.len() - 1;
        let range_len = Self::MAX_COLORS / range_count;
        let mut colors = [Rgb(0, 0, 0); Self::MAX_COLORS];

        for i in 0..Self::MAX_COLORS {
            let range_index = (i / range_len).min(range_count - 1);
            let alpha = (i - range_index * range_len) as f64 / range_len as f64;
            let (start, end) = (gradient[range_index], gradient[range_index + 1]);
            colors[i] = Self::interpolate_rgb(start, end, alpha);
        }

        Self { colors }
    }

    pub fn color(&self, value: u8) -> Rgb {
        self.colors[value as usize]
    }

    fn interpolate_rgb(start: Rgb, end: Rgb, alpha: f64) -> Rgb {
        if alpha >= 1.0 {
            return end;
        }
        Rgb(
            Self::interpolate_u8(start.0, end.0, alpha),
            Self::interpolate_u8(start.1, end.1, alpha),
            Self::interpolate_u8(start.2, end.2, alpha),
        )
    }

    fn interpolate_u8(start: u8, end: u8, alpha: f64) -> u8 {
        let start = start as f64;
        let end = end as f64;
        (start + (end - start) * alpha) as u8
    }
}

pub mod palettes {
    use super::{Palette, Rgb};
    use once_cell::sync::Lazy;

    pub static CYAN: Lazy<Palette> = Lazy::new(|| {
        Palette::from_gradient(&[
            Rgb(0, 0, 0),
            Rgb(0, 35, 66),
            Rgb(0, 56, 89),
            Rgb(0, 78, 114),
            Rgb(0, 102, 139),
            Rgb(0, 127, 165),
            Rgb(0, 152, 187),
            Rgb(0, 177, 205),
            Rgb(0, 203, 220),
            Rgb(0, 229, 231),
            Rgb(0, 255, 238),
        ])
    });

    pub static BLUE_GREEN: Lazy<Palette> = Lazy::new(|| {
        Palette::from_gradient(&[
            Rgb(97, 179, 255),
            Rgb(33, 10, 127),
            Rgb(5, 136, 218),
            Rgb(11, 204, 49),
            Rgb(33, 253, 43),
            Rgb(0, 0, 0),
        ])
    });

    pub static YELLOW_RED: Lazy<Palette> = Lazy::new(|| {
        Palette::from_gradient(&[
            Rgb(0, 0, 0),
            Rgb(250, 255, 0),
            Rgb(255, 168, 0),
            Rgb(255, 77, 0),
            Rgb(153, 41, 41),
            Rgb(0, 0, 0),
        ])
    });

    pub static RAINBOW: Lazy<Palette> = Lazy::new(|| {
        Palette::from_gradient(&[
            Rgb(255, 255, 255),
            Rgb(255, 0, 0),
            Rgb(255, 255, 0),
            Rgb(0, 255, 255),
            Rgb(127, 127, 255),
            Rgb(255, 0, 255),
            Rgb(0, 0, 255),
            Rgb(0, 255, 0),
            Rgb(0, 0, 0),
        ])
    });
}
