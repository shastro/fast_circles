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

pub struct RectBound {
    pub pos: Vec2,
    pub kind: BoundaryType,
    pub width: f32,
    pub height: f32,
}

impl Boundary for RectBound {
    fn kind(&self) -> BoundaryType {
        return self.kind;
    }
    fn apply_inner_constraint(&self, ball: &mut Ball) {
        let half_width = self.width / 2.;
        let half_height = self.height / 2.;
        let bot = self.pos.y - half_height;
        let top = self.pos.y + half_height;
        let left = self.pos.x - half_width;
        let right = self.pos.x + half_width;

        if ball.pos.y < bot {
            ball.pos.y = self.pos.y - half_height + ball.radius;
        }
        if ball.pos.y > top {
            // y collide
            ball.pos.y = self.pos.y + half_height - ball.radius;
        }

        if ball.pos.x < left {
            ball.pos.x = self.pos.x - half_width + ball.radius;
        }
        if ball.pos.x > right {
            // x collide
            ball.pos.x = self.pos.x + half_width - ball.radius;
        }
    }
    fn apply_outer_constraint(&self, ball: &mut Ball) {
        let half_width = self.width / 2.;
        let half_height = self.height / 2.;
        let bot = self.pos.y - half_height;
        let top = self.pos.y + half_height;
        let left = self.pos.x - half_width;
        let right = self.pos.x + half_width;

        if (ball.pos.y > bot && ball.pos.y < top) && (ball.pos.x > left && ball.pos.x < right) {
            if ball.pos.y < self.pos.y {
                ball.pos.y = bot - ball.radius;
            }
            if ball.pos.y > self.pos.y {
                ball.pos.y = top + ball.radius;
            }
            if ball.pos.x < self.pos.x {
                ball.pos.x = left - ball.radius;
            }
            if ball.pos.x > self.pos.x {
                ball.pos.x = right + ball.radius;
            }
        }
    }
    fn detect_inner_collision(&self, ball: &Ball) -> bool {
        let bot = self.pos.y - self.height / 2.;
        let top = self.pos.y + self.height / 2.;
        let left = self.pos.x - self.width / 2.;
        let right = self.pos.x + self.width / 2.;

        ball.pos.y < bot || ball.pos.y > top || ball.pos.x < left || ball.pos.x > right
    }
    fn detect_outer_collision(&self, ball: &Ball) -> bool {
        let bot = self.pos.y - self.height / 2.;
        let top = self.pos.y + self.height / 2.;
        let left = self.pos.x - self.width / 2.;
        let right = self.pos.x + self.width / 2.;

        (ball.pos.y > bot && ball.pos.y < top) && (ball.pos.x > left && ball.pos.x < right)
    }
    fn draw(&self, draw: &Draw) {
        draw.rect()
            .xy(self.pos)
            .width(self.width)
            .no_fill()
            .stroke_weight(1.)
            .stroke(WHITE)
            .height(self.height);
    }
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
