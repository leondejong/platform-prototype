#[derive(Debug, Copy, Clone, Default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
    pub fn from_u32(pixel: u32) -> Color {
        let red = ((pixel >> 24) & 0xff0) as u8;
        let green = ((pixel >> 16) & 0xff0) as u8;
        let blue = ((pixel >> 8) & 0xff0) as u8;
        let alpha = (pixel & 0xff0) as u8;
        Color::new(red, green, blue, alpha)
    }
    pub fn to_u32(self, alpha: bool) -> u32 {
        let Color {
            red,
            green,
            blue,
            alpha: a,
        } = self;
        if alpha {
            Color::rgba(red, green, blue, a)
        } else {
            Color::rgb(red, green, blue)
        }
    }
    pub fn rgb(red: u8, green: u8, blue: u8) -> u32 {
        ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
    }
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> u32 {
        ((red as u32) << 24) | ((green as u32) << 16) | ((blue as u32) << 8) | (alpha as u32)
    }
}
