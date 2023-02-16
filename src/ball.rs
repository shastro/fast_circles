use nannou::prelude::*;

pub struct Ball {
    pub prev_pos: Vec2,
    pub pos: Vec2,
    pub radius: f32,
    pub acc: Vec2,
}

impl Ball {
    pub fn update(&mut self, dt: f32) {
        let term1 = 2. * self.pos - self.prev_pos;
        self.prev_pos = self.pos;
        self.pos = term1 + self.acc * (dt.powi(2) as f32);
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc = acc;
    }
}
