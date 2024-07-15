use std::collections::BTreeMap;

use image::error::ImageError;
use image::DynamicImage;

use super::rectangle::Rectangle;

#[derive(Debug, Clone, Default)]
pub struct Image {
    x: f32,
    y: f32,
    image: DynamicImage,
}

impl Image {
    pub fn new(x: f32, y: f32, image: DynamicImage) -> Self {
        Self { x, y, image }
    }
    pub fn update(&mut self, x: f32, y: f32, image: DynamicImage) {
        self.x = x;
        self.y = y;
        self.image = image;
    }
    pub fn from_uri(x: f32, y: f32, uri: &str) -> Result<Self, ImageError> {
        let i = image::open(uri)?;

        Ok(Self { x, y, image: i })
    }
    pub fn from_bytes(x: f32, y: f32, buffer: &[u8]) -> Result<Self, ImageError> {
        let i = image::load_from_memory(buffer)?;

        Ok(Self { x, y, image: i })
    }
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    pub fn set_x_y(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn width(&self) -> u32 {
        self.image.width()
    }
    pub fn height(&self) -> u32 {
        self.image.height()
    }
    pub fn bytes(&self) -> &[u8] {
        self.image.as_bytes()
    }
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Image {
        Self {
            x: self.x,
            y: self.y,
            image: self.image.crop(x, y, width, height),
        }
    }
    pub fn crop_rectangle(&mut self, rectangle: Rectangle) -> Image {
        Self {
            x: self.x,
            y: self.y,
            image: self.image.crop(
                rectangle.x as u32,
                rectangle.y as u32,
                rectangle.width as u32,
                rectangle.height as u32,
            ),
        }
    }
    pub fn sprite_to_texture_map(
        sprite: &mut Image,
        width: u32,
        height: u32,
        indices: &[u32],
    ) -> BTreeMap<u32, Image> {
        let mut map = BTreeMap::new();
        for index in indices {
            let x = (index * width) % sprite.width();
            let y = ((index * width) / sprite.width()) * height;
            let image = sprite.crop(x, y, width, height);
            map.insert(*index, image);
        }
        map
    }
    pub fn sprite_to_texture_list(
        sprite: &mut Image,
        width: u32,
        height: u32,
        indices: &[u32],
    ) -> Vec<Image> {
        let map = Image::sprite_to_texture_map(sprite, width, height, indices);
        map.into_values().collect()
    }
}
