use crate::ball::*;
use crate::boundary::*;
use crate::partition::*;
use nannou::color::Rgba;
use nannou::image::io::Reader;
use nannou::image::DynamicImage;
use nannou::prelude::*;
use random::Source;
use std::cell::RefCell;
use std::io::Cursor;
use std::time::Instant;

pub enum DetectMode {
    SpatialPartition,
    Slow,
}

pub enum ColorMode {
    Velocity,
    Collide,
    Index,
}

pub struct Solver {
    pub gravity: Vec2,
    pub balls: Vec<RefCell<Ball>>,
    pub boundaries: Vec<Box<dyn Boundary>>,
    pub substeps: usize,
    pub hash: SpatialHash,
    pub detect_mode: DetectMode,
    pub colormap: Vec<Rgba>,
    pub pixel_scale: f32,
}

impl Solver {
    pub fn update(&mut self, dt: f32) {
        let now = Instant::now();
        let subdt = dt / (self.substeps as f32);
        for _ in 0..self.substeps {
            self.apply_gravity();
            self.apply_boundaries();
            match self.detect_mode {
                DetectMode::SpatialPartition => self.solve_grid_collisions(),
                DetectMode::Slow => self.solve_collisions(),
            }
            self.update_positions(subdt);
        }
        // println!("{}", 1. / now.elapsed().as_secs_f32());
    }

    pub fn set_image_colors(&mut self, image: &mut DynamicImage) {
        let pixel_size_real = self.pixel_scale;
        let image = image.to_rgba8();
        let width_real = image.width() as f32 * pixel_size_real;
        let height_real = image.height() as f32 * pixel_size_real;
        let image_pos = Vec2::new(-width_real / 2., height_real / 2.);
        let num_x_pixels = image.width();
        let num_y_pixels = image.height();
        let pixel_size_real = pixel_size_real as f32;

        for i in 0..self.balls.len() {
            let mut ball = self.balls[i].borrow_mut();
            // Get relative to corner an index based on the pixel size
            let mut py = (((height_real / 2.0) - ball.pos.y) / pixel_size_real) as usize;
            let mut px = (((width_real / 2.0) + ball.pos.x) / pixel_size_real) as usize;
            px = px.clamp(0, (num_x_pixels - 1) as usize);
            py = py.clamp(0, (num_y_pixels - 1) as usize);
            let rgba = image.get_pixel(px as u32, py as u32);
            let r = rgba[0] as f32 / 255.;
            let g = rgba[1] as f32 / 255.;
            let b = rgba[2] as f32 / 255.;
            let a = rgba[3] as f32 / 255.;
            // println!("{} {} {} {}", r, g, b, a);
            ball.color = Hsv::from(Rgb::new(r, g, b));
            self.colormap[i] = Rgba::new(r, g, b, a);
        }
    }

    pub fn restart(&mut self) {
        for cell in self.hash.grid.iter_mut() {
            // cell.truncate(0);
            cell.clear();
        }
        self.balls.clear();
    }

    fn check_cell_collisions(&mut self, cell_1_idx: (usize, usize), cell_2_idx: (usize, usize)) {
        // Loop over indicies to check for collisions in this kernel
        let cell_1 = self.hash.grid.get(cell_1_idx.0, cell_1_idx.1).unwrap();
        let cell_2 = self.hash.grid.get(cell_2_idx.0, cell_2_idx.1).unwrap();
        for current_idx in cell_1 {
            let mut did_collide = false;
            for other_idx in cell_2 {
                if current_idx != other_idx {
                    unsafe {
                        let mut current_ball = self.balls.get_unchecked(*current_idx).borrow_mut();
                        let mut other_ball = self.balls.get_unchecked(*other_idx).borrow_mut();
                        if Ball::detect_pair_collide(&current_ball, &other_ball) {
                            // current_ball.color = Hsv::new(0., 1., 1.);
                            // other_ball.color = Hsv::new(0., 1., 1.);
                            did_collide = true;
                            Ball::resolve_pair_collide(&mut current_ball, &mut other_ball);
                        }
                    }
                }
            }
            if !did_collide {
                unsafe {
                    let mut current_ball = self.balls.get_unchecked(*current_idx).borrow_mut();
                    // current_ball.color = Hsv::new(0., 0., 1.);
                }
            }
        }
    }

    fn solve_grid_collisions(&mut self) {
        // TODO: Refactor this
        let mut to_check: Vec<usize> = Vec::new();
        // Insert into grid
        // println!("{} {}", rows, cols);
        for cell in self.hash.grid.iter_mut() {
            // cell.truncate(0);
            cell.clear();
        }

        // let now = Instant::now();
        // Hash the balls
        self.balls.iter().enumerate().for_each(|(i, ball)| {
            self.hash.hash(ball.borrow().pos, i);
        });

        // println!("Time hash {}", now.elapsed().as_secs_f32());
        // Detect collisions

        let now = Instant::now();
        let (rows, cols) = self.hash.grid.size();
        for cr in 1..rows - 1 {
            for cc in 1..cols - 1 {
                // Loop around each cell
                to_check.clear();
                let current_cell_idx = (cr, cc);
                for i in 0..3 {
                    for j in 0..3 {
                        // True index
                        let current_row = cr + i - 1;
                        let current_col = cc + j - 1;
                        // println!("Here before clamp{} {}", current_row, current_col);
                        // Clamp to valid index
                        let current_row = current_row.clamp(0, rows);
                        let current_col = current_col.clamp(0, cols);
                        // println!("Here {} {}", current_row, current_col);
                        let other_cell_idx = (current_row, current_col);
                        let other_cell = self.hash.grid.get(current_row, current_col).unwrap();
                        self.check_cell_collisions(current_cell_idx, other_cell_idx);
                    }
                }
            }
        }
    }

    // println!("Time collide {}", 1. / now.elapsed().as_secs_f32());
    fn update_positions(&mut self, dt: f32) {
        self.balls
            .iter_mut()
            .for_each(|x| x.borrow_mut().update(dt));
    }

    fn apply_boundaries(&mut self) {
        // for _ in 0..self.substeps {
        self.boundaries.iter().for_each(|bound| {
            self.balls.iter_mut().for_each(|ball| match bound.kind() {
                BoundaryType::Inner => {
                    if bound.detect_inner_collision(&mut ball.borrow_mut()) {
                        bound.apply_inner_constraint(&mut ball.borrow_mut())
                    }
                }
                BoundaryType::Outer => {
                    if bound.detect_outer_collision(&mut ball.borrow_mut()) {
                        bound.apply_outer_constraint(&mut ball.borrow_mut())
                    }
                }
            })
        });
        // }
    }
    fn apply_gravity(&mut self) {
        self.balls
            .iter_mut()
            // .for_each(|x| x.borrow_mut().accelerate(self.gravity));
            .for_each(|x| {
                let pos = x.borrow().pos;
                x.borrow_mut().accelerate(-2000000000. * pos.normalize())
            });
    }

    pub fn solve_collisions(&mut self) {
        for _ in 0..self.substeps {
            for i in 0..self.balls.len() {
                let (before, since) = self.balls.split_at_mut(i);
                let (mut current, after) = since.split_first_mut().unwrap();
                let mut did_collide = false;
                for mut other in before.iter_mut().chain(after) {
                    if Ball::detect_pair_collide(&current.borrow_mut(), &other.borrow_mut()) {
                        // current.borrow_mut().color = Hsv::new(0., 1., 1.);
                        // other.borrow_mut().color = Hsv::new(0., 1., 1.);
                        did_collide = true;
                        Ball::resolve_pair_collide(
                            &mut current.borrow_mut(),
                            &mut other.borrow_mut(),
                        );
                    }
                }
                if !did_collide {
                    // current.borrow_mut().color = Hsv::new(0., 0., 1.);
                }
            }
        }
    }

    pub fn draw(&self, draw: &Draw) {
        self.balls.iter().for_each(|ball| {
            let ball = ball.borrow();
            draw.ellipse()
                .color(ball.color)
                .xy(ball.pos)
                .radius(ball.radius);
        });

        self.boundaries.iter().for_each(|bound| bound.draw(draw));
        // self.hash.draw(draw);
    }

    pub fn init_balls(ball_radius: f32) -> Vec<RefCell<Ball>> {
        let mut vec_balls = Vec::<RefCell<Ball>>::new();
        let max = 0; // try 60
        let hue_step = 360. / ((max * max) as f32);
        let mut i = 0.;
        let max_radius = ball_radius;
        let min_radius = 3.0;
        let radius_range = max_radius - min_radius;
        let xpos = 0. * ball_radius;
        let ypos = 0. * ball_radius;
        let mut source = random::default(42);
        for x in 1..max {
            let xd = ((x as f32) * 2. * ball_radius) - (max / 2) as f32 * 2. * ball_radius + xpos; // and minus 5
            for y in 1..max {
                let rand_radius = radius_range * source.read_f64() as f32 + min_radius;
                let yd =
                    ((y as f32) * 2. * ball_radius) - (max / 2) as f32 * 2. * ball_radius + ypos;
                vec_balls.push(RefCell::new(Ball {
                    prev_pos: Vec2::new(xd, yd),
                    pos: Vec2::new(xd, yd),
                    radius: ball_radius,
                    color: Hsv::new(i * hue_step, 1., 1.),
                    acc: Vec2::ZERO,
                }));
                i += 1.;
            }
        }
        vec_balls
    }
}
