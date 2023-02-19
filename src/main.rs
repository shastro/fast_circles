use nannou::prelude::*;
mod ball;
mod boundary;
mod partition;
mod solver;
mod spawn;
use boundary::*;
use nannou::image::io::Reader;
use nannou::image::DynamicImage;
use partition::*;
use solver::*;
use spawn::*;
use std::fs;
use std::io::Cursor;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    solver: Solver<CircleBound>,
    timestep: f32,
    ball_radius: f32,
    spawners: Vec<LinearSpawner>,
    color_image: DynamicImage,
}

fn model(_app: &App) -> Model {
    let ball_radius = 5.;
    Model {
        ball_radius,
        color_image: Reader::open("cat.jpg").unwrap().decode().unwrap(),
        spawners: vec![
            LinearSpawner::new(Vec2::new(150., 200.), PI, 3, 5., 10, false, 6074),
            // LinearSpawner::new(Vec2::new(-150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(-150., 150.), 0., 6, 4., 5, false),
        ],
        solver: Solver {
            gravity: Vec2::new(0.0, -20000.),
            balls: Solver::<CircleBound>::init_balls(ball_radius),
            hash: SpatialHash::new(ball_radius, 1500., 1500.),
            substeps: 8,
            detect_mode: DetectMode::SpatialPartition,
            boundaries: vec![
                CircleBound {
                    pos: Vec2::new(0., 0.),
                    radius: 400.,
                    kind: BoundaryType::Inner,
                },
                // CircleBound {
                //     pos: Vec2::new(2000., 2000.),
                //     radius: 100.,
                //     kind: BoundaryType::Outer,
                // },
                // CircleBound {
                //     pos: Vec2::new(2000., 2000.),
                //     radius: 80.,
                //     kind: BoundaryType::Outer,
                // },
                // CircleBound {
                //     pos: Vec2::new(0., -200.),
                //     radius: 50.,
                //     kind: BoundaryType::Outer,
                // },
            ],
        },

        timestep: 0.01,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.solver.update(_model.timestep);
    let frames = _app.elapsed_frames() as usize;
    println!("{}", frames);
    let time = (frames as f32) * _model.timestep;
    let w = -2. * 3.14159 / 20.;
    let r = 400. - 20.;
    // let mouse_bound = &mut _model.solver.boundaries[1];
    // mouse_bound.pos.x = _app.mouse.x;
    // mouse_bound.pos.y = _app.mouse.y;
    // let move_bound = &mut _model.solver.boundaries[1];
    // move_bound.pos.x = r * (time * w).cos();
    // move_bound.pos.y = r * (time * w).sin();
    // let next_bound = &mut _model.solver.boundaries[2];
    // next_bound.pos.x = r * (time * w + PI).cos();
    // next_bound.pos.y = r * (time * w + PI).sin();
    let first_bound = &mut _model.solver.boundaries[0];
    // first_bound.radius = r + 100. * (time * w).sin();

    for (i, spawner) in _model.spawners.iter_mut().enumerate() {
        spawner.update(
            &mut _model.solver.balls,
            _model.ball_radius,
            time,
            frames,
            |t| PI,
        )
    }

    _model.solver.set_image_colors(&mut _model.color_image);
    if frames % 2000 == 0 {
        _model.solver.restart();
        for spawner in _model.spawners.iter_mut() {
            spawner.reset()
        }
    }
    // _model.spawners[0].set_pos(_app.mouse.position());
    // let inner_bound = &mut _model.solver.boundaries[3];
    // inner_bound.radius = 70. + 70. * (time * 7. * w).sin();
    // _model.solver.gravity = Vec2::new(
    //     -100. * (_app.time as f32 * w).cos(),
    //     -100. * (_app.time as f32 * w).sin(),
    // );
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    frame.clear(BLACK);

    _model.solver.draw(&draw);
    draw.to_frame(_app, &frame).unwrap();
}
