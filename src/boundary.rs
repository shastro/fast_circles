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
    fn detect_collision(&self, ball: &Ball) -> bool;
}

struct RectBound {}

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
        ball.pos = normal * self.radius + normal * (0.5 * ball.radius);
    }
    fn apply_outer_constraint(&self, ball: &mut Ball) {}
    fn detect_collision(&self, ball: &Ball) -> bool {
        let sum_radii = self.radius + ball.radius;
        (ball.pos - self.pos).length_squared() < sum_radii.pow(2)
    }
}
