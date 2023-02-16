use crate::ball::*;
use nannou::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum BoundaryType {
    Inner,
    Outer,
}

pub trait Boundary {
    fn kind(&self) -> BoundaryType;
    fn apply_outer_constraint(&self, ball: &mut Ball);
    fn apply_inner_constraint(&self, ball: &mut Ball);
    fn detect_inner_collision(&self, ball: &Ball) -> bool;
    fn detect_outer_collision(&self, ball: &Ball) -> bool;
    fn draw(&self, draw: &Draw);
}

pub struct CircleBound {
    pub pos: Vec2,
    pub radius: f32,
    pub kind: BoundaryType,
}

impl Boundary for CircleBound {
    fn kind(&self) -> BoundaryType {
        return self.kind;
    }
    fn apply_inner_constraint(&self, ball: &mut Ball) {
        let normal = (ball.pos - self.pos).normalize();
        ball.pos = self.pos + normal * (self.radius - ball.radius);
    }
    fn apply_outer_constraint(&self, ball: &mut Ball) {
        let normal = (ball.pos - self.pos).normalize();
        ball.pos = self.pos + normal * (self.radius + ball.radius);
    }
    fn detect_inner_collision(&self, ball: &Ball) -> bool {
        (ball.pos - self.pos).length_squared() > (self.radius - ball.radius).pow(2)
    }
    fn detect_outer_collision(&self, ball: &Ball) -> bool {
        (ball.pos - self.pos).length_squared() < (self.radius + ball.radius).pow(2)
    }

    fn draw(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius)
            .no_fill()
            .stroke_weight(1.)
            .stroke(WHITE)
            .xy(self.pos);
    }
}
