use crate::ball::*;
use crate::boundary::*;
use nannou::prelude::*;

pub struct Solver<T: Boundary> {
    pub gravity: Vec2,
    pub balls: Vec<Ball>,
    pub boundaries: Vec<T>,
    pub substeps: usize,
}

impl<T: Boundary> Solver<T> {
    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_boundaries();
        self.solve_collisions();
        self.update_positions(dt);
    }

    fn update_positions(&mut self, dt: f32) {
        self.balls.iter_mut().for_each(|x| x.update(dt));
    }

    fn apply_boundaries(&mut self) {
        self.boundaries.iter().for_each(|bound| {
            self.balls.iter_mut().for_each(|ball| match bound.kind() {
                BoundaryType::Inner => {
                    if bound.detect_inner_collision(ball) {
                        bound.apply_inner_constraint(ball)
                    }
                }
                BoundaryType::Outer => {
                    if bound.detect_outer_collision(ball) {
                        bound.apply_outer_constraint(ball)
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

    pub fn solve_collisions(&mut self) {
        for _ in 0..self.substeps {
            for i in 0..self.balls.len() {
                let (before, since) = self.balls.split_at_mut(i);
                let (mut current, after) = since.split_first_mut().unwrap();
                for (mut other_left, mut other_right) in before.iter_mut().zip(after) {
                    if (Ball::detect_pair_collide(&current, &other_left)) {
                        Ball::resolve_pair_collide(&mut current, &mut other_left);
                    }
                    if (Ball::detect_pair_collide(&current, &other_right)) {
                        Ball::resolve_pair_collide(&mut current, &mut other_right);
                    }
                }
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.balls.iter().for_each(|ball| {
            draw.ellipse().color(WHITE).xy(ball.pos).radius(ball.radius);
        });

        self.boundaries.iter().for_each(|bound| bound.draw(draw));
    }

    pub fn init_balls(ball_radius: f32) -> Vec<Ball> {
        let mut vec_balls = Vec::<Ball>::new();
        for x in 1..5 {
            let xd = ((x as f32) * 2. * ball_radius) - 5. * 2. * ball_radius;
            for y in 1..5 {
                let yd = ((y as f32) * 2. * ball_radius) - 5. * 2. * ball_radius;
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
