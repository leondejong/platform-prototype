use crate::graphics::image::Image;
use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;

pub fn render(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    point: &Point,
    rectangle: &Rectangle,
    image: &Image,
) {
    let rx = rectangle.x.round() as i32;
    let ry = rectangle.y.round() as i32;

    if rx < 0 || ry < 0 {
        return;
    }

    let rw = rectangle.width.round() as i32;
    let rh = rectangle.height.round() as i32;

    let rxw = rx + rw;
    let ryh = ry + rh;

    let w = image.width() as i32;
    let h = image.height() as i32;

    if rxw > w || ryh > h {
        return;
    }

    let width = width as i32;
    let height = height as i32;

    let x = point.x.round() as i32;
    let y = point.y.round() as i32;

    if x >= width || y >= height {
        return;
    }

    let bytes = image.bytes();
    let size = buffer.len();
    let length = rw * rh * 4;

    let mut point = 0;

    loop {
        let i = point / 4;

        let x = x + i % rw;
        let y = y + i / rw;

        let index = (x + y * width) * 4;

        let ix = rx + i % rw;
        let iy = ry + i / rw;

        let cursor = (ix + iy * w) * 4;

        if index + 3 > size as i32 {
            break;
        }

        if x >= 0 && y >= 0 && x < width && y < height {
            if bytes[cursor as usize + 3] > 0 {
                buffer[index as usize + 0] = bytes[cursor as usize + 0];
                buffer[index as usize + 1] = bytes[cursor as usize + 1];
                buffer[index as usize + 2] = bytes[cursor as usize + 2];
                buffer[index as usize + 3] = bytes[cursor as usize + 3];
            }
        }

        point += 4;

        if point >= length {
            break;
        }
    }
}
