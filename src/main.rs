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
use nannou::ui::color::DARK_CHARCOAL;
use partition::*;
use solver::*;
use spawn::*;
use std::fs;
use std::io::Cursor;
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
}

fn model(_app: &App) -> Model {
    let ball_radius = 3.;
    let image_name = "cat2.jpg";
    let spawn_period = 1;
    let num_rows = 15;
    let num_balls = 10000;
    let frames_for_color_reset = (num_balls / num_rows) * spawn_period + 1000000;
    // let frames_for_color_reset = 1000000;
    let mut model = Model {
        ball_radius,
        frames_for_color_reset,
        boundary_time: 0.,
        sync_frames: 0,
        sim_runs: 0,
        color_image: Reader::open(image_name).unwrap().decode().unwrap(),
        spawners: vec![
            LinearSpawner::new(
                Vec2::new(0., 350.),
                -PI - 0.05,
                spawn_period,
                1.,
                num_rows,
                false,
                num_balls,
            ),
            // LinearSpawner::new(
            //     Vec2::new(0., -350.),
            //     PI / 2.,
            //     1,
            //     3.,
            //     num_rows,
            //     false,
            //     num_balls,
            // ),
            // LinearSpawner::new(Vec2::new(-150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(-150., 150.), 0., 6, 4., 5, false),
        ],
        solver: Solver {
            gravity: Vec2::new(0.0, -4000000000.),
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
                Box::new(CircleBound {
                    pos: Vec2::new(0., 0.),
                    kind: BoundaryType::Outer,
                    radius: 95.,
                    sink: false,
                }),
                // Box::new(RectBound {
                //     pos: Vec2::new(0., 0.),
                //     kind: BoundaryType::Outer,
                //     width: 300.,
                //     height: 100.,
                //     sink: false,
                // }),
            ],
        },

        timestep: 0.000011,
    };

    // Create funnel
    // let mut sign = 1.;
    // let gap = 23.;
    // for i in 0..32 {
    //     let count = i / 2;
    //     model.solver.boundaries.push(Box::new(CircleBound {
    //         pos: Vec2::new(
    //             sign * gap + ((count + 1) as f32 * sign as f32) * 10.,
    //             count as f32 * 30.,
    //         ),
    //         radius: 15.,
    //         kind: BoundaryType::Outer,
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
    // let mouse_bound = &mut _model.solver.boundaries[1];

    let spawner = &mut _model.spawners[0];
    // mouse_bound.set_pos(_app.mouse.position());
    // let move_bound = &mut _model.solver.boundaries[1];
    // move_bound.pos.x = r * (_model.boundary_time * w).cos();
    // move_bound.pos.y = r * (_model.boundary_time * w).sin();
    // let next_bound = &mut _model.solver.boundaries[2];
    // next_bound.pos.x = r * (_model.boundary_time * w + PI).cos();
    // next_bound.pos.y = r * (_model.boundary_time * w + PI).sin();
    // let first_bound = &mut _model.solver.boundaries[0];
    // first_bound.radius = r + 20. * (_model.boundary_time * 5. * w).sin();

    for (i, spawner) in _model.spawners.iter_mut().enumerate() {
        spawner.update(
            &mut _model.solver.balls,
            _model.ball_radius,
            _model.boundary_time,
            _model.sync_frames,
            |t| (0.1 * (t * w + (i as f32) * PI).sin()),
            &mut _model.solver.colormap,
        )
    }

    if (frames == _model.frames_for_color_reset) && frames > 0 {
        _model.solver.set_image_colors(&mut _model.color_image);
        _model.boundary_time = 0.;
        _model.sync_frames = 0;
        _model.solver.restart();
        for spawner in _model.spawners.iter_mut() {
            spawner.reset()
        }
    }

    if frames % _model.frames_for_color_reset == 0 && frames > 0 {
        _model.sim_runs += 1;
    }

    _model
        .solver
        .balls
        .retain(|b| b.borrow().pos.length_squared() > (100f32).pow(2.));

    let time_ran = now.elapsed();
    let target_time = std::time::Duration::from_millis(16);
    if time_ran < target_time {
        std::thread::sleep(target_time - time_ran);
    }
    println!("\nFPS {}", 1. / now.elapsed().as_secs_f32());
    // _model.spawners[0].set_pos(_app.mouse.position());
    // let inner_bound = &mut _model.solver.boundaries[3];
    // inner_bound.radius = 70. + 70. * (time * 7. * w).sin();
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    frame.clear(BLACK);

    _model.solver.draw(&draw);
    draw.to_frame(_app, &frame).unwrap();
}
