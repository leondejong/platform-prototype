use super::color::Color;
use super::image::Image;
use super::point::Point;
use super::rectangle::Rectangle;

#[derive(Debug, Clone, Default)]
pub enum Graphic {
    #[default]
    None,
    Background {
        color: Color,
    },
    Point {
        point: Point,
        color: Color,
    },
    Rectangle {
        rectangle: Rectangle,
        color: Color,
    },
    Image {
        image: Image,
    },
    Sprite {
        image: Image,
        point: Point,
        rectangle: Rectangle,
    },
}

impl Graphic {
    pub fn none() -> Self {
        Graphic::None
    }
    pub fn background(color: Color) -> Self {
        Graphic::Background { color }
    }
    pub fn point(point: Point, color: Color) -> Self {
        Graphic::Point { point, color }
    }
    pub fn rectangle(rectangle: Rectangle, color: Color) -> Self {
        Graphic::Rectangle { rectangle, color }
    }
    pub fn image(image: Image) -> Self {
        Graphic::Image { image }
    }
    pub fn image_x_y(image: Image, x: f32, y: f32) -> Self {
        let mut image = image;
        image.set_x(x);
        image.set_y(y);
        Graphic::Image { image }
    }
    pub fn cropped_image(image: &mut Image, rectangle: Rectangle) -> Self {
        let image = image.crop_rectangle(rectangle);
        Graphic::Image { image }
    }
    pub fn sprite(image: Image, point: Point, rectangle: Rectangle) -> Self {
        Graphic::Sprite {
            image,
            point,
            rectangle,
        }
    }
}

impl Graphic {
    pub fn set_x_y(&mut self, x: f32, y: f32) {
        match self {
            Graphic::Point { point, .. } => {
                point.x = x;
                point.y = y;
            }
            Graphic::Rectangle { rectangle, .. } => {
                rectangle.x = x;
                rectangle.y = y;
            }
            Graphic::Image { image } => {
                image.set_x(x);
                image.set_y(y);
            }
            Graphic::Sprite { point, .. } => {
                point.x = x;
                point.y = y;
            }
            _ => {}
        }
    }
}
