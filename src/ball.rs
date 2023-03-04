use nannou::color::Hsv;
use nannou::prelude::*;

pub struct Ball {
    pub prev_pos: Vec2,
    pub pos: Vec2,
    pub radius: f32,
    pub acc: Vec2,
    pub color: Hsv,
}

impl Ball {
    pub fn detect_pair_collide(a: &Ball, b: &Ball) -> bool {
        let sum_radii_sqr = (a.radius + b.radius).pow(2);
        let dpos = a.pos - b.pos;
        dpos.length_squared() < sum_radii_sqr
    }

    pub fn resolve_pair_collide(a: &mut Ball, b: &mut Ball) {
        let axis = (a.pos - b.pos).normalize();
        let overlap = (a.radius + b.radius) - (a.pos - b.pos).length();
        a.pos = a.pos + axis * (0.5 * overlap);
        b.pos = b.pos - axis * (0.5 * overlap);
    }
    pub fn update(&mut self, dt: f32) {
        let vel = self.pos - self.prev_pos;
        self.color = Hsv::new(0.1 * vel.length_squared() / 1. * 480., 1., 1.);
        // self.radius = 10. * (vel.length_squared() / 2.);
        // self.radius = self.radius.clamp(2., 4.);
        self.prev_pos = self.pos;
        self.pos = self.pos + vel + self.acc * (dt * dt);
        self.acc = Vec2::ZERO;
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acc += acc;
    }
}
