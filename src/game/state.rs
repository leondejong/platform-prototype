use std::collections::BTreeMap;

use crate::display::window::Graphics;

use crate::graphics::animation::Animations;
use crate::graphics::image::Image;
use crate::graphics::point::Point;
use crate::graphics::rectangle::Rectangle;
use crate::graphics::tile::Tile;

use super::data::subject_animations;
use super::data::{BACKGROUND, ENVIRONMENT, MAP};
use super::data::{DENSITY, IMPULSE, JUMP, MASS};
use super::data::{DISSIPATION, FRICTION, GRAVITY, RESISTANCE};
use super::data::{ENV_HEIGHT, ENV_WIDTH, ENV_X, ENV_Y, TILE_HEIGHT, TILE_WIDTH};
use super::data::{FPS, HEIGHT, MAX, RATIO, STEP, TITLE, WIDTH};
use super::data::{PATTERN, SPRITE_X, SPRITE_Y, SUB_HEIGHT, SUB_WIDTH, SUB_X, SUB_Y};

use super::logic::{render_graphics, update_state};

// State setup
#[derive(Default)]
pub struct State {
    pub conf: Configuration, // Game
    pub env: Environment,    // Level
    pub sub: Subject,        // Player
}

// Game setup
#[derive(Default)]
pub struct Configuration {
    pub title: String,    // Window title
    pub width: f32,       // Window width
    pub height: f32,      // Window height
    pub step: f32,        // Frame time step
    pub ratio: f32,       // Frame time ratio
    pub delta: f32,       // Frame delta time
    pub fps: f32,         // Frame count per second
    pub max: f32,         // Frame max delta time
    pub time: f32,        // Frame total time
    pub accumulator: f32, // Frame time accumulator
    pub up: bool,         // Key up
    pub down: bool,       // Key down
    pub left: bool,       // Key left
    pub right: bool,      // Key right
    pub jump: bool,       // Key jump
}

// Level properties
#[derive(Default)]
pub struct Environment {
    pub x: f32,                         // Level x
    pub y: f32,                         // Level y
    pub width: f32,                     // Level width
    pub height: f32,                    // Level height
    pub gravity: f32,                   // Level gravity
    pub friction: f32,                  // Level friction
    pub resistance: f32,                // Level air resistance
    pub dissipation: Point,             // Level dissipation
    pub pattern: Image,                 // Level pattern
    pub background: Image,              // Level background
    pub spritesheet: Image,             // Level spritesheet
    pub textures: BTreeMap<u32, Image>, // Level textures
    pub tiles: Vec<Tile>,               // Level tiles
}

// Player properties
#[derive(Default)]
pub struct Subject {
    pub x: f32,                 // Player x
    pub y: f32,                 // Player y
    pub width: f32,             // Player width
    pub height: f32,            // Player heigth
    pub force: Point,           // Player force
    pub velocity: Point,        // Player velocity
    pub acceleration: Point,    // Player acceleration
    pub direction: Point,       // Player direction
    pub previous: Point,        // Player previous
    pub impulse: Point,         // Player impulse
    pub interpolation: Point,   // Player interpolation
    pub mass: f32,              // Player mass
    pub density: f32,           // Player density
    pub jump: f32,              // Player jump impulse
    pub contact: bool,          // Player contact
    pub lock: bool,             // Player lock
    pub spritesheet: Image,     // Player spritesheet
    pub animations: Animations, // Player animations
    pub offset: Point,          // Player offset
}

impl State {
    pub fn new() -> Self {
        let mut state = Self {
            conf: Configuration::new(),
            env: Environment::new(),
            sub: Subject::new(),
        };

        state.initialize();

        state
    }
    fn initialize(&mut self) {
        self.sub.mass = self.sub.width * self.sub.height * self.sub.density;
        self.env.friction = (1.0 - self.env.friction).powf(self.conf.ratio);
        self.env.resistance = (1.0 - self.env.resistance).powf(self.conf.ratio);
        self.env.dissipation.x = (1.0 - self.env.dissipation.x).powf(self.conf.ratio);
        self.env.dissipation.y = (1.0 - self.env.dissipation.y).powf(self.conf.ratio);
    }
    pub fn view(&self) -> Rectangle {
        Rectangle::new(self.env.x, self.env.y, self.conf.width, self.conf.height)
    }
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            title: TITLE.into(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            step: STEP,
            ratio: RATIO,
            delta: STEP,
            fps: FPS,
            max: MAX,
            time: 0.0,
            accumulator: 0.0,
            up: false,
            right: false,
            down: false,
            left: false,
            jump: false,
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        let fb = Image::from_bytes;
        let st = Image::sprite_to_texture_map;
        let mt = Tile::map_to_tiles;

        let message = "Sprite should contain valid image data";
        let indices = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        let pattern = fb(0.0, 0.0, PATTERN).expect(message);
        let background = fb(0.0, 0.0, BACKGROUND).expect(message);
        let mut spritesheet = fb(0.0, 0.0, ENVIRONMENT).expect(message);

        let textures = st(&mut spritesheet, TILE_WIDTH, TILE_HEIGHT, indices);
        let tiles = mt(MAP, TILE_WIDTH, TILE_HEIGHT);

        Self {
            x: ENV_X as f32,
            y: ENV_Y as f32,
            width: ENV_WIDTH as f32,
            height: ENV_HEIGHT as f32,
            gravity: GRAVITY,
            friction: FRICTION,
            resistance: RESISTANCE,
            dissipation: Point {
                x: DISSIPATION,
                y: 0.0,
            },
            pattern,
            background,
            spritesheet,
            textures,
            tiles,
        }
    }
}

impl Subject {
    pub fn new() -> Self {
        let x = SUB_X as f32;
        let y = SUB_Y as f32;
        let width = SUB_WIDTH as f32;
        let height = SUB_HEIGHT as f32;

        let offset = Point::new(SPRITE_X as f32, SPRITE_Y as f32);

        let (spritesheet, mut animations) = subject_animations();

        animations.set("run_right");

        Self {
            x,
            y,
            width,
            height,
            force: Point::new(0.0, 0.0),
            velocity: Point::new(0.0, 0.0),
            acceleration: Point::new(0.0, 0.0),
            direction: Point::new(0.0, 0.0),
            previous: Point::new(0.0, 0.0),
            impulse: Point::new(IMPULSE, 0.0),
            interpolation: Point::new(0.0, 0.0),
            mass: MASS,
            density: DENSITY,
            jump: JUMP,
            contact: false,
            lock: false,
            spritesheet,
            animations,
            offset,
        }
    }
}

impl Graphics for State {
    fn input(&mut self, active: bool, key: &str) {
        match key {
            "s" => self.conf.left = active,
            "f" => self.conf.right = active,
            "e" => self.conf.up = active,
            "d" => self.conf.down = active,
            "j" => self.conf.jump = active,
            _ => {}
        }
    }
    fn update(&mut self, time: f32, delta: f32, fps: f32) {
        update_state(self, time, delta, fps);
    }
    fn render(&mut self, buffer: &mut [u8], width: u32, height: u32) {
        render_graphics(self, buffer, width, height);
    }
}
