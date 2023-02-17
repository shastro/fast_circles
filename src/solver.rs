use crate::ball::*;
use crate::boundary::*;
use grid::*;
use nannou::prelude::*;
use random::Source;
use std::cell::RefCell;
use std::time::Instant;

pub struct Solver<T: Boundary> {
    pub gravity: Vec2,
    pub balls: Vec<RefCell<Ball>>,
    pub boundaries: Vec<T>,
    pub substeps: usize,
    pub hash: SpatialHash,
}

pub struct SpatialHash {
    grid: Grid<Vec<usize>>,
    resolution: f32,
    win_width: f32,
    win_height: f32,
}

impl SpatialHash {
    pub fn new(radius: f32, win_width: f32, win_height: f32) -> Self {
        let res = 2. * radius;
        let nrow = (win_height / res) as usize + 1;
        let ncol = (win_width / res) as usize + 1;
        // let nrow = (1000. / res) as usize + 1;
        // let ncol = (1000. / res) as usize + 1;
        let grid = Grid::new(ncol, nrow);
        SpatialHash {
            grid: grid,
            resolution: res,
            win_height: win_height,
            win_width: win_width,
        }
    }

    pub fn hash(&mut self, pos: Vec2, index: usize) {
        println!("{:?}", pos);
        let mut px = 0.;
        let mut py = 0.;
        if pos.y > 0. {
            py = ((self.win_height / 2.0) - pos.y) / self.resolution;
        } else {
            py = ((self.win_height / 2.0) + (pos.y.abs())) / self.resolution;
        }

        if pos.x > 0. {
            px = ((self.win_width / 2.0) + pos.x) / self.resolution;
        } else {
            px = ((self.win_width / 2.0) - pos.x.abs()) / self.resolution;
        }
        println!("{} {}", px, py);
        let list = self.grid.get_mut(px as usize, py as usize);
        if list.is_some() {
            list.unwrap().push(index);
        }
    }

    pub fn draw(&self, draw: &Draw) {
        // Detect collisions
        let (rows, cols) = self.grid.size();
        // println!("{} {}", rows, cols);
        for cr in 0..rows {
            for cc in 0..cols {
                if (self.grid.get(cr, cc).is_some()) {
                    let items: &Vec<usize> = self.grid.get(cr, cc).unwrap();
                    if !items.is_empty() {
                        draw.rect()
                            .xy(Vec2::new(
                                (cr as f32 * self.resolution) - self.win_width / 2.0,
                                (cc as f32 * self.resolution) - self.win_height / 2.0,
                            ))
                            .wh(Vec2::new(self.resolution, self.resolution))
                            .stroke(RED)
                            .stroke_weight(0.5)
                            .rgba(1., 0., 0., 1.);
                    } else {
                        draw.rect()
                            .xy(Vec2::new(
                                (cr as f32 * self.resolution) - self.win_width / 2.0,
                                (cc as f32 * self.resolution) - self.win_height / 2.0,
                            ))
                            .wh(Vec2::new(self.resolution, self.resolution))
                            .stroke(WHITE)
                            .stroke_weight(0.5)
                            .no_fill();
                    }
                }
            }
        }
    }
}

impl<T: Boundary> Solver<T> {
    pub fn update(&mut self, dt: f32) {
        self.apply_gravity();
        self.apply_boundaries();
        let now = Instant::now();
        // self.solve_grid_collisions();
        self.solve_collisions();
        println!("{}", 1. / now.elapsed().as_secs_f32());
        // self.solve_collisions();
        self.update_positions(dt / (self.substeps as f32));
    }
    fn solve_grid_collisions(&mut self) {
        // TODO: Refactor this
        let mut to_check: Vec<usize> = Vec::new();
        for _ in 0..self.substeps {
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
            // println!("{} {}", rows, cols);
            for cr in 1..rows - 1 {
                for cc in 1..cols - 1 {
                    // Loop around each cell
                    to_check.clear();
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
                            let in_cell = self.hash.grid.get_mut(current_row, current_col);
                            if in_cell.is_some() {
                                to_check.append(&mut in_cell.unwrap());
                            }
                        }
                    }

                    if (to_check.len() > 0) {
                        println!("{} {} {:?}", cr, cc, to_check.len());
                    }
                    // Loop over indicies to check for collisions in this kernel
                    for i in 0..to_check.len() {
                        // Split index array in such a way such that
                        // you have no two indicies in the same place
                        // I could do this more simply but I did it this way ig.
                        let (before, since) = to_check.split_at(i);
                        let (current_idx, after) = since.split_first().unwrap();
                        for other_idx in before.iter().chain(after) {
                            // Time to get two mutable references at once!
                            unsafe {
                                // Before = [1, i-1], since = [i, len]
                                //
                                let mut current_ball =
                                    self.balls.get_unchecked(*current_idx).borrow_mut();

                                let mut other_ball =
                                    self.balls.get_unchecked(*other_idx).borrow_mut();
                                if Ball::detect_pair_collide(&current_ball, &other_ball) {
                                    Ball::resolve_pair_collide(&mut current_ball, &mut other_ball);
                                }
                            }
                        }
                    }
                }
            }

            // println!("Time collide {}", 1. / now.elapsed().as_secs_f32());
        }
    }
    fn update_positions(&mut self, dt: f32) {
        self.balls
            .iter_mut()
            .for_each(|x| x.borrow_mut().update(dt));
    }

    fn apply_boundaries(&mut self) {
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
    }
    fn apply_gravity(&mut self) {
        self.balls
            .iter_mut()
            // .for_each(|x| x.borrow_mut().accelerate(self.gravity));
            .for_each(|x| {
                let pos = x.borrow().pos;
                x.borrow_mut().accelerate(-2000. * pos.normalize())
            });
    }

    pub fn solve_collisions(&mut self) {
        for _ in 0..self.substeps {
            for i in 0..self.balls.len() {
                let (before, since) = self.balls.split_at_mut(i);
                let (mut current, after) = since.split_first_mut().unwrap();
                for mut other in before.iter_mut().chain(after) {
                    if Ball::detect_pair_collide(&current.borrow(), &other.borrow()) {
                        Ball::resolve_pair_collide(
                            &mut current.borrow_mut(),
                            &mut other.borrow_mut(),
                        );
                    }
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
        let max = 35; // try 60
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
