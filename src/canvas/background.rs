use crate::graphics::color::Color;

pub fn render(buffer: &mut [u8], color: &Color) {
    let length = buffer.len();

    let mut point = 0;

    loop {
        buffer[point + 0] = color.red;
        buffer[point + 1] = color.green;
        buffer[point + 2] = color.blue;
        buffer[point + 3] = color.alpha;

        point += 4;

        if point >= length {
            break;
        }
    }
}
