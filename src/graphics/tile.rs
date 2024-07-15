use std::str;

use super::point::Point;
use super::rectangle::Rectangle;

pub struct Tile {
    pub id: u32,
    pub position: Point,
    pub boundary: Rectangle,
}

impl Tile {
    pub fn new(id: u32, position: Point, boundary: Rectangle) -> Self {
        Self {
            id,
            position,
            boundary,
        }
    }
    pub fn map_to_tiles(map: &str, width: u32, height: u32) -> Vec<Tile> {
        let width: f32 = width as f32;
        let height: f32 = height as f32;

        let lines = map.split("\n");
        let mut tiles = Vec::new();

        let ascii = "Map should contain valid ASCII characters"; // 7 bits, first 128 characters
        let hex = "Values should be in hexadecimal format";

        for (y, line) in lines.enumerate() {
            for (x, value) in line
                .as_bytes()
                .chunks(2)
                .map(|c| str::from_utf8(c).expect(ascii))
                .enumerate()
            {
                if value != "  " {
                    let x = x as f32 * width;
                    let y = y as f32 * height;

                    let id = u32::from_str_radix(&value.to_string(), 16).expect(hex);

                    let position = Point::new(x, y);
                    let boundary = Rectangle::new(x, y, width, height);

                    let tile = Tile::new(id, position, boundary);

                    tiles.push(tile);
                }
            }
        }
        tiles
    }
}
