use crate::ball::*;
use float_ord::FloatOrd;
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
    fn set_pos(&mut self, new_pos: Vec2);
    fn draw(&self, draw: &Draw);
    fn sink(&self) -> bool;
}

pub struct RectBound {
    pub pos: Vec2,
    pub kind: BoundaryType,
    pub width: f32,
    pub height: f32,
    pub sink: bool,
}

impl Boundary for RectBound {
    fn sink(&self) -> bool {
        self.sink
    }
    fn kind(&self) -> BoundaryType {
        return self.kind;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }
    fn apply_inner_constraint(&self, ball: &mut Ball) {
        let half_width = self.width / 2.;
        let half_height = self.height / 2.;
        let bot = self.pos.y - half_height;
        let top = self.pos.y + half_height;
        let left = self.pos.x - half_width;
        let right = self.pos.x + half_width;

        if ball.pos.y < bot + ball.radius {
            ball.pos.y = bot + ball.radius;
        }
        if ball.pos.y > top - ball.radius {
            // y collide
            ball.pos.y = top - ball.radius;
        }

        if ball.pos.x < left + ball.radius {
            ball.pos.x = left + ball.radius;
        }
        if ball.pos.x > right - ball.radius {
            // x collide
            ball.pos.x = right - ball.radius;
        }
    }
    fn apply_outer_constraint(&self, ball: &mut Ball) {
        let half_width = self.width / 2.;
        let half_height = self.height / 2.;
        let bot = self.pos.y - half_height;
        let top = self.pos.y + half_height;
        let left = self.pos.x - half_width;
        let right = self.pos.x + half_width;

        let dbot = (ball.pos.y - bot).abs();
        let dtop = (ball.pos.y - top).abs();
        let dleft = (ball.pos.x - left).abs();
        let dright = (ball.pos.x - right).abs();

        enum Side {
            Bot,
            Top,
            Left,
            Right,
        }

        let dists = vec![
            (Side::Bot, FloatOrd(dbot)),
            (Side::Top, FloatOrd(dtop)),
            (Side::Left, FloatOrd(dleft)),
            (Side::Right, FloatOrd(dright)),
        ];

        let min_dist = dists.iter().min_by_key(|(tp, val)| val).unwrap();

        match min_dist.0 {
            Side::Bot => ball.pos.y = bot - 2. * ball.radius,
            Side::Top => ball.pos.y = top + 2. * ball.radius,
            Side::Left => ball.pos.x = left - 2. * ball.radius,
            Side::Right => ball.pos.x = right + 2. * ball.radius,
        }
    }
    fn detect_inner_collision(&self, ball: &Ball) -> bool {
        let bot = self.pos.y - self.height / 2. + ball.radius;
        let top = self.pos.y + self.height / 2. - ball.radius;
        let left = self.pos.x - self.width / 2. + ball.radius;
        let right = self.pos.x + self.width / 2. - ball.radius;

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
    pub sink: bool,
}

impl Boundary for CircleBound {
    fn sink(&self) -> bool {
        self.sink
    }
    fn kind(&self) -> BoundaryType {
        return self.kind;
    }
    fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
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
