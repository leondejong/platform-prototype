use std::collections::BTreeMap;
use std::thread;
use std::time::{Duration, Instant};

use super::image::Image;

#[derive(Debug, Clone, Default)]
pub struct Animations {
    name: String,
    map: BTreeMap<String, Animation>,
}

impl Animations {
    pub fn new() -> Self {
        Self {
            name: "".into(),
            map: BTreeMap::new(),
        }
    }
    pub fn add(&mut self, name: &str, animation: Animation) {
        self.map.insert(name.into(), animation);
    }
    pub fn remove(&mut self, name: &str) {
        self.map.remove(name);
    }
    pub fn set(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn get(&self) -> Option<&Animation> {
        self.map.get(&self.name)
    }
    pub fn get_mut(&mut self) -> Option<&mut Animation> {
        self.map.get_mut(&self.name)
    }
    pub fn get_by(&self, name: &str) -> Option<&Animation> {
        self.map.get(name.into())
    }
    pub fn get_mut_by(&mut self, name: &str) -> Option<&mut Animation> {
        self.map.get_mut(name.into())
    }
    pub fn get_names(&self) -> Vec<&String> {
        self.map.keys().collect::<Vec<&String>>()
    }
    pub fn consecutive_frame(&mut self, x: f32, y: f32) -> &Image {
        let animation = self.get_mut().unwrap();

        animation.set_active(true);
        animation.consecutive();

        let frame = animation.frame_mut();

        frame.set_x_y(x, y);

        frame
    }
}

#[derive(Debug, Clone, Default)]
pub struct Animation {
    frames: Vec<Image>,
    index: usize,
    active: bool,
    reverse: bool,
    step: f32,
    delta: f32,
    fps: f32,
}

impl Animation {
    pub fn new(frames: Vec<Image>) -> Self {
        if frames.len() < 1 {
            panic!("Frame map should contain at least one frame");
        }
        Self {
            frames,
            index: 0,
            active: false,
            reverse: false,
            step: 1.0 / 60.0,
            delta: 0.0,
            fps: 0.0,
        }
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn active(&self) -> bool {
        self.active
    }
    pub fn fps(&self) -> f32 {
        self.fps
    }
    pub fn length(&self) -> usize {
        self.frames.len()
    }
    pub fn set_index(&mut self, index: usize) {
        if index < self.frames.len() {
            self.index = index;
        } else {
            eprintln!("Index {} must be smaller than {}", index, self.frames.len());
        }
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }
    pub fn set_fps(&mut self, fps: f32) {
        self.step = 1.0 / fps;
    }
    pub fn set_frames(&mut self, frames: Vec<Image>) {
        self.index = 0;
        self.frames = frames;
    }
    pub fn reset(&mut self) {
        self.index = 0;
        self.active = false;
        self.reverse = false;
        self.delta = 0.0;
        self.fps = 0.0;
    }
    pub fn frame(&self) -> &Image {
        &self.frames[self.index]
    }
    pub fn frame_mut(&mut self) -> &mut Image {
        &mut self.frames[self.index]
    }
    pub fn next(&mut self) {
        if self.index == self.frames.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
    pub fn previous(&mut self) {
        if self.index == 0 {
            self.index = self.frames.len() - 1;
        } else {
            self.index -= 1;
        }
    }
    pub fn consecutive(&mut self) {
        if !self.active || self.frames.len() < 2 {
            return;
        };
        if self.reverse {
            self.previous();
        } else {
            self.next();
        }
    }
    pub fn start(&mut self) {
        self.active = true;
        self.cycle();
    }
    pub fn stop(&mut self) {
        self.active = false;
        self.delta = 0.0;
        self.fps = 0.0;
    }
    fn cycle(&mut self) {
        let duration = Duration::from_secs_f32(self.step as f32);
        let mut now = Instant::now();
        let mut elapsed;
        let mut delta;

        while !self.active {
            elapsed = now.elapsed();

            if duration > elapsed {
                delta = duration - elapsed;
            } else {
                delta = Duration::new(0, 0);
            }

            thread::sleep(delta);

            self.delta = (delta + elapsed).as_secs_f32();
            self.fps = 1.0 / self.delta;

            now = Instant::now();

            self.consecutive();
        }
    }
}
