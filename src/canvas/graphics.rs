use crate::graphics::graphic::Graphic;

use super::background;
use super::image;
use super::point;
use super::rectangle;
use super::sprite;

pub fn render(buffer: &mut [u8], width: u32, height: u32, graphics: &Vec<&Graphic>) {
    for graphic in graphics.iter() {
        match graphic {
            Graphic::Background { color } => {
                background::render(buffer, color);
            }
            Graphic::Point { point, color } => {
                point::render(buffer, width, height, point, color);
            }
            Graphic::Rectangle { rectangle, color } => {
                rectangle::render(buffer, width, height, rectangle, color);
            }
            Graphic::Image { image } => {
                image::render(buffer, width, height, image);
            }
            Graphic::Sprite {
                image,
                point,
                rectangle,
            } => {
                sprite::render(buffer, width, height, point, rectangle, image);
            }
            _ => {}
        }
    }
}
