use crate::graphics::image::Image;

pub fn render(buffer: &mut [u8], width: u32, height: u32, image: &Image) {
    let bytes = image.bytes();

    let width = width as i32;
    let height = height as i32;

    let x = image.x().round() as i32;
    let y = image.y().round() as i32;

    if x >= width || y >= height {
        return;
    }

    let w = image.width() as i32;
    let h = image.height() as i32;

    let length = w * h * 4;
    let size = buffer.len();

    let mut point = 0;

    loop {
        let i = point / 4;
        let x = x + i % w;
        let y = y + i / w;

        let index = (x + y * width) * 4;

        if index + 3 > size as i32 {
            break;
        }

        if x >= 0 && y >= 0 && x < width && y < height {
            if bytes[point as usize + 3] > 0 {
                buffer[index as usize + 0] = bytes[point as usize + 0];
                buffer[index as usize + 1] = bytes[point as usize + 1];
                buffer[index as usize + 2] = bytes[point as usize + 2];
                buffer[index as usize + 3] = bytes[point as usize + 3];
            }
        }

        point += 4;

        if point >= length {
            break;
        }
    }
}
