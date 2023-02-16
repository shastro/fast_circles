use crate::ball::*;
use crate::boundary::*;
use nannou::prelude::*;

pub struct Solver<T: Boundary> {
    pub gravity: Vec2,
    pub balls: Vec<Ball>,
    pub boundaries: Vec<T>,
}

impl<T: Boundary> Solver<T> {
    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.update_positions(dt);
    }

    fn update_positions(&mut self, dt: f32) {
        self.balls.iter_mut().for_each(|x| x.update(dt));
    }

    fn apply_boundaries(&mut self) {
        self.boundaries.iter().for_each(|bound| {
            self.balls.iter_mut().for_each(|ball| {
                if bound.detect_collision(ball) {
                    match bound.kind() {
                        INNER => bound.apply_inner_constraint(ball),
                        OUTER => bound.apply_outer_constraint(ball),
                    }
                }
            })
        });
    }
    fn apply_gravity(&mut self) {
        self.balls
            .iter_mut()
            .for_each(|x| x.accelerate(self.gravity));
    }

    pub fn draw(&self, draw: &Draw) {
        self.balls.iter().for_each(|ball| {
            draw.ellipse().color(WHITE).xy(ball.pos);
        });
    }

    pub fn init_balls(ball_radius: f32) -> Vec<Ball> {
        let mut vec_balls = Vec::<Ball>::new();
        for x in 1..10 {
            let xd = (x as f32) * 50. * ball_radius;
            for y in 1..10 {
                let yd = (y as f32) * 50. * ball_radius;
                vec_balls.push(Ball {
                    prev_pos: Vec2::new(xd, yd),
                    pos: Vec2::new(xd, yd),
                    radius: ball_radius,
                    acc: Vec2::ZERO,
                });
            }
        }
        vec_balls
    }
}
