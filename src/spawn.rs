use crate::ball::*;
use nannou::prelude::*;
use std::cell::RefCell;

pub trait Spawner {
    fn set_pos(&mut self, pos: Vec2);
    fn update<D: Fn(f32) -> f32>(
        &mut self,
        ball_vec: &mut Vec<RefCell<Ball>>,
        ball_radius: f32,
        time: f32,
        frame_count: usize,
        angle_driver: D,
        color_map: &mut Vec<Rgba>,
    ) -> u32;
    fn reset(&mut self);
}

// pub type DriverFunc = Fn(f32) -> f32;

pub struct LinearSpawner {
    pos: Vec2,
    angle: f32,
    spawn_period: usize,
    spawn_velocity: f32,
    rows: usize,
    mirror: bool,
    max_spawn: usize,
    spawn_count: usize,
}

impl LinearSpawner {
    pub fn new(
        pos: Vec2,
        angle: f32,
        spawn_period: usize,
        spawn_velocity: f32,
        rows: usize,
        mirror: bool,
        max_spawn: usize,
    ) -> Self {
        LinearSpawner {
            pos,
            angle,
            spawn_period,
            spawn_velocity,
            rows,
            mirror,
            max_spawn,
            spawn_count: 0,
        }
    }
}

impl Spawner for LinearSpawner {
    fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    fn reset(&mut self) {
        self.spawn_count = 0;
    }

    fn update<D: Fn(f32) -> f32>(
        &mut self,
        vec_balls: &mut Vec<RefCell<Ball>>,
        ball_radius: f32,
        time: f32,
        frame_count: usize,
        angle_driver: D,
        color_map: &mut Vec<Rgba>,
    ) -> u32 {
        let angle_offset = self.angle + (angle_driver)(time);
        // println!("Spawn Count {} Frame {}", self.spawn_count, frame_count);

        let mut num_spawned_now = 0;
        if frame_count % self.spawn_period == 0 {
            let normal = Vec2::new(1., 0.).rotate(angle_offset).normalize();
            let tangent = Vec2::new(1., 0.).rotate(angle_offset + PI / 2.).normalize();
            let spacing = 2. * ball_radius;

            // Spawing each ball
            for i in 0..self.rows {
                // By default set color to a hue based on spawn_count
                let mut color = Rgba::from(Hsv::new(self.spawn_count as f32 * 25. / 360., 1., 1.));

                let spawn_pos = self.pos - (self.rows as f32 / 2. * spacing * tangent)
                    + (i as f32) * spacing * tangent;

                // Color map is the mapping defined by the user, such as an image
                if color_map.get(self.spawn_count).is_some() {
                    color = *color_map.get(self.spawn_count).unwrap();
                } else {
                    color_map.push(color);
                }

                let color_hsv = Hsv::from(color);

                // Push a ball
                if self.spawn_count < self.max_spawn {
                    vec_balls.push(RefCell::new(Ball {
                        prev_pos: spawn_pos,
                        pos: spawn_pos + self.spawn_velocity * normal,
                        radius: ball_radius,
                        color: color_hsv,
                        acc: Vec2::ZERO,
                    }));

                    self.spawn_count += 1;
                    num_spawned_now += 1;
                }
            }
        }
        num_spawned_now
    }
}
