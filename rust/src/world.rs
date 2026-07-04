use crate::{
    memory,
    observer,
    swing,
    weather,
};

pub struct World {
    pub wind: i32,
    pub swing_height: f32,
    pub memory: i32,
    pub observer: String,
}

impl World {
    pub fn new() -> Self {
        Self {
            wind: 0,
            swing_height: 0.0,
            memory: 0,
            observer: "Nobody".into(),
        }
    }

    pub fn step(&mut self) {
        weather::update(self);
        swing::update(self);
        observer::update(self);
        memory::update(self);
    }
}
