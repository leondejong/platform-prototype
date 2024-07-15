use crate::graphics::color::Color;
use crate::graphics::point::Point;

pub fn render(buffer: &mut [u8], width: u32, height: u32, point: &Point, color: &Color) {
    let x = point.x.round();
    let y = point.y.round();

    if x < 0.0 || x >= width as f32 || y < 0.0 || y >= height as f32 {
        return;
    }

    let index = (x as usize + y as usize * width as usize) * 4;

    buffer[index + 0] = color.red;
    buffer[index + 1] = color.green;
    buffer[index + 2] = color.blue;
    buffer[index + 3] = color.alpha;
}
