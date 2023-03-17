use nannou::prelude::*;
mod ball;
use std::time::Instant;
mod boundary;
mod partition;
mod solver;
mod spawn;
use boundary::*;
use nannou::image::io::Reader;
use nannou::image::{DynamicImage, GenericImageView};
use partition::*;
use solver::*;
use spawn::*;
use std::fs;
use std::io::Cursor;
use std::thread;
use std::thread::sleep;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    solver: Solver,
    timestep: f32,
    ball_radius: f32,
    spawners: Vec<LinearSpawner>,
    color_image: DynamicImage,
    frames_for_color_reset: usize,
    boundary_time: f32,
    sync_frames: usize,
    sim_runs: usize,
    fps: f32,
    ball_count: usize,
}

fn model(_app: &App) -> Model {
    let ball_radius = 5.;
    let image_name = "cat2.jpg";
    let spawn_period = 1;
    // let num_rows = 880 / (2 * ball_radius as usize);
    let num_rows = 10;
    let num_balls = 7550;
    let frames_for_color_reset = (num_balls / num_rows) * spawn_period + 100;
    // let frames_for_color_reset = 1000000;
    let mut model = Model {
        fps: 0.,
        ball_count: 0,
        ball_radius,
        frames_for_color_reset,
        boundary_time: 0.,
        sync_frames: 0,
        sim_runs: 0,
        color_image: Reader::open(image_name).unwrap().decode().unwrap(),
        spawners: vec![
            LinearSpawner::new(
                Vec2::new(0., 440. - 100.),
                -PI / 2.,
                spawn_period,
                2.,
                num_rows,
                false,
                num_balls,
            ),
            // LinearSpawner::new(
            //     Vec2::new(0., -350.),
            //     PI / 2.,
            //     spawn_period,
            //     2.2,
            //     num_rows,
            //     false,
            //     num_balls,
            // ),
            // LinearSpawner::new(Vec2::new(-150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(-150., 150.), 0., 6, 4., 5, false),
        ],
        solver: Solver {
            gravity: Vec2::new(0.0, 0.),
            balls: Solver::init_balls(ball_radius),
            hash: SpatialHash::new(ball_radius, 900., 900.),
            substeps: 8,
            pixel_scale: 900.
                / Reader::open(image_name)
                    .unwrap()
                    .decode()
                    .unwrap()
                    .dimensions()
                    .0 as f32,
            detect_mode: DetectMode::SpatialPartition,
            colormap: vec![],
            boundaries: vec![
                Box::new(RectBound {
                    pos: Vec2::new(0., 0.),
                    kind: BoundaryType::Inner,
                    width: 880.,
                    height: 880.,
                    sink: false,
                }),
                // Box::new(CircleBound {
                //     pos: Vec2::new(0., 0.),
                //     kind: BoundaryType::Outer,
                //     radius: 50.,
                //     sink: false,
                // }),
                // Box::new(CircleBound {
                //     pos: Vec2::new(0., -220.),
                //     kind: BoundaryType::Outer,
                //     radius: 10.,
                //     sink: false,
                // }),
                Box::new(CircleBound {
                    pos: Vec2::new(-150., 0.),
                    kind: BoundaryType::Outer,
                    radius: 50.,
                    sink: false,
                }),
                // Box::new(CircleBound {
                //     pos: Vec2::new(150., 0.),
                //     kind: BoundaryType::Outer,
                //     radius: 50.,
                //     sink: false,
                // }),
                // Box::new(RectBound {
                //     pos: Vec2::new(0., 0.),
                //     kind: BoundaryType::Outer,
                //     width: 300.,
                //     height: 100.,
                //     sink: false,
                // }),
            ],
        },

        timestep: 0.0000000011,
    };

    // Create funnel also abstract later
    // let mut sign = 1.;
    // let gap = 70.;
    // for i in 0..24 {
    //     let count = i / 2;
    //     model.solver.boundaries.push(Box::new(CircleBound {
    //         pos: Vec2::new(
    //             sign * gap + ((count + 1) as f32 * sign as f32) * 10.,
    //             -200. + count as f32 * 30.,
    //         ),
    //         radius: 15.,
    //         kind: BoundaryType::Outer,
    //         sink: false,
    //     }));
    //     sign *= -1.;
    // }
    model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Critical Updates
    let now = Instant::now();
    _model.solver.update(_model.timestep);
    let frames = _app.elapsed_frames() as usize;
    if _model.sim_runs < 2 {
        _model.sync_frames += 1;
    }
    _model.boundary_time += 0.01;

    // Animations
    let f = 1.;
    let w = -2. * 3.14159 * f;
    let r = 400. - 20.;
    let mouse_bound = &mut _model.solver.boundaries[1];
    mouse_bound.set_pos(_app.mouse.position());

    // let spawner = &mut _model.spawners[0];
    // let move_bound = &mut _model.solver.boundaries[1];
    // move_bound.pos.x = r * (_model.boundary_time * w).cos();
    // move_bound.pos.y = r * (_model.boundary_time * w).sin();
    // let next_bound = &mut _model.solver.boundaries[2];
    // next_bound.pos.x = r * (_model.boundary_time * w + PI).cos();
    // next_bound.pos.y = r * (_model.boundary_time * w + PI).sin();
    // let first_bound = &mut _model.solver.boundaries[0];
    // first_bound.radius = r + 20. * (_model.boundary_time * 5. * w).sin();
    // Spawning section
    let mut total_spawns_frame = 0;
    for (i, spawner) in _model.spawners.iter_mut().enumerate() {
        total_spawns_frame += spawner.update(
            &mut _model.solver.balls,
            _model.ball_radius,
            _model.boundary_time,
            _model.sync_frames,
            // |t| 0.0,
            |t| (0.15 * (t * w).sin()),
            &mut _model.solver.colormap,
        );
    }

    // Color reset
    if (frames == _model.frames_for_color_reset) && frames > 0 {
        _model.solver.set_image_colors(&mut _model.color_image);
        _model.boundary_time = 0.;
        _model.sync_frames = 0;
        _model.ball_count = 0;
        _model.solver.restart();
        for spawner in _model.spawners.iter_mut() {
            spawner.reset()
        }
    }

    // Count runs to enable correct resets of state
    if frames % _model.frames_for_color_reset == 0 && frames > 0 {
        _model.sim_runs += 1;
    }

    // TODO: Abstract over this
    // _model
    //     .solver
    //     .balls
    //     .retain(|b| b.borrow().pos.length_squared() > (100f32).pow(2.));

    // _model.solver.balls.retain(|b| b.borrow().pos.y > -420.);
    // Update count
    _model.ball_count = _model.solver.balls.len();

    // Timing
    let time_ran = now.elapsed();
    let target_time = std::time::Duration::from_millis(16);
    if time_ran < target_time {
        std::thread::sleep(target_time - time_ran);
    }
    _model.fps = 1. / now.elapsed().as_secs_f32();
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    frame.clear(BLACK);

    _model.solver.draw(&draw);

    draw.text(format!("FPS {:.0} Ball Count {}", _app.fps(), _model.ball_count).as_str())
        .font_size(30)
        .width(800.)
        .xy(Vec2::new(-300., 460.));

    draw.to_frame(_app, &frame).unwrap();
}
