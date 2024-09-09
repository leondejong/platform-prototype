use std::fmt;

use crate::graphics::animation::{Animation, Animations};
use crate::graphics::image::Image;

// Window properties
pub const WIDTH: u32 = 384;
pub const HEIGHT: u32 = 288;
pub const RESIZABLE: bool = false;
pub const FILTER: bool = false;
pub const SCALE: f32 = 1.0;
pub const TITLE: &str = "Platform Game";

// Game properties
pub const FPS: f32 = 60.0; // Physics frame rate
pub const STEP: f32 = 1.0 / FPS;
pub const RATIO: f32 = 60.0 / FPS;
pub const MAX: f32 = 0.0625;

// Level properties
pub const GRAVITY: f32 = 40.0;
pub const FRICTION: f32 = 0.0;
pub const RESISTANCE: f32 = 0.05;
pub const DISSIPATION: f32 = 0.075;

// Player properties
pub const MASS: f32 = 64.0;
pub const DENSITY: f32 = 0.25;
pub const IMPULSE: f32 = 36.0;
pub const JUMP: f32 = 16.0;

// Tile properties
pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;

// Environment properties
pub const ENV_X: i32 = 0;
pub const ENV_Y: i32 = 0;
pub const ENV_WIDTH: u32 = 768;
pub const ENV_HEIGHT: u32 = 576;

// Subject properties
pub const SUB_X: i32 = 32;
pub const SUB_Y: i32 = 32;
pub const SUB_WIDTH: u32 = 16;
pub const SUB_HEIGHT: u32 = 32;

// Sprite properties
pub const SPRITE_X: i32 = -4;
pub const SPRITE_Y: i32 = 0;
pub const SPRITE_WIDTH: u32 = 24;
pub const SPRITE_HEIGHT: u32 = 32;

// Player and level data
pub const MAP: &str = include_str!("../../assets/level.map");
pub const PATTERN: &[u8] = include_bytes!("../../assets/pattern.png");
pub const BACKGROUND: &[u8] = include_bytes!("../../assets/background.png");
pub const ENVIRONMENT: &[u8] = include_bytes!("../../assets/environment.png");
pub const CHARACTER: &[u8] = include_bytes!("../../assets/character.png");

// Character Sprite Animation Type
#[derive(Debug)]
pub enum Sprite {
    FacingLeft,
    FacingRight,
    FacingUp,
    FacingDown,
    RunningLeft,
    RunningRight,
    RunningUp,
    RunningDown,
    AirLeft,
    AirRight,
    AirUp,
    AirDown,
}

impl Sprite {
    pub fn str(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Create player character animations from spritesheet
pub fn subject_animations() -> (Image, Animations) {
    let width = SPRITE_WIDTH;
    let height = SPRITE_HEIGHT;

    let fb = Image::from_bytes;
    let st = Image::sprite_to_texture_list;

    let message = "Sprite should contain valid image data";

    let mut animations = Animations::new();
    let mut spritesheet = fb(SPRITE_X as f32, SPRITE_Y as f32, CHARACTER).expect(message);

    let indices_facing_left = [16];
    let indices_facing_right = [24];
    let indices_facing_up = [8];
    let indices_facing_down = [0];
    let indices_air_left = [22];
    let indices_air_right = [30];
    let indices_air_up = [14];
    let indices_air_down = [2];
    let indices_run_left = [16, 17, 18, 19, 20, 21, 22, 23];
    let indices_run_right = [24, 25, 26, 27, 28, 29, 30, 31];
    let indices_run_up = [8, 9, 10, 11, 12, 13, 14, 15];
    let indices_run_down = [0, 1, 3, 4, 5, 6, 7];

    let frames_facing_left = st(&mut spritesheet, width, height, &indices_facing_left);
    let frames_facing_right = st(&mut spritesheet, width, height, &indices_facing_right);
    let frames_facing_up = st(&mut spritesheet, width, height, &indices_facing_up);
    let frames_facing_down = st(&mut spritesheet, width, height, &indices_facing_down);
    let frames_air_left = st(&mut spritesheet, width, height, &indices_air_left);
    let frames_air_right = st(&mut spritesheet, width, height, &indices_air_right);
    let frames_air_up = st(&mut spritesheet, width, height, &indices_air_up);
    let frames_air_down = st(&mut spritesheet, width, height, &indices_air_down);
    let frames_run_left = st(&mut spritesheet, width, height, &indices_run_left);
    let frames_run_right = st(&mut spritesheet, width, height, &indices_run_right);
    let frames_run_up = st(&mut spritesheet, width, height, &indices_run_up);
    let frames_run_down = st(&mut spritesheet, width, height, &indices_run_down);

    let animation_facing_left = Animation::new(frames_facing_left);
    let animation_facing_right = Animation::new(frames_facing_right);
    let animation_facing_up = Animation::new(frames_facing_up);
    let animation_facing_down = Animation::new(frames_facing_down);
    let animation_air_left = Animation::new(frames_air_left);
    let animation_air_right = Animation::new(frames_air_right);
    let animation_air_up = Animation::new(frames_air_up);
    let animation_air_down = Animation::new(frames_air_down);
    let animation_run_left = Animation::new(frames_run_left);
    let animation_run_right = Animation::new(frames_run_right);
    let animation_run_up = Animation::new(frames_run_up);
    let animation_run_down = Animation::new(frames_run_down);

    animations.add(&Sprite::FacingLeft.str(), animation_facing_left);
    animations.add(&Sprite::FacingRight.str(), animation_facing_right);
    animations.add(&Sprite::FacingUp.str(), animation_facing_up);
    animations.add(&Sprite::FacingDown.str(), animation_facing_down);
    animations.add(&Sprite::AirLeft.str(), animation_air_left);
    animations.add(&Sprite::AirRight.str(), animation_air_right);
    animations.add(&Sprite::AirUp.str(), animation_air_up);
    animations.add(&Sprite::AirDown.str(), animation_air_down);
    animations.add(&Sprite::RunningLeft.str(), animation_run_left);
    animations.add(&Sprite::RunningRight.str(), animation_run_right);
    animations.add(&Sprite::RunningUp.str(), animation_run_up);
    animations.add(&Sprite::RunningDown.str(), animation_run_down);

    (spritesheet, animations)
}
