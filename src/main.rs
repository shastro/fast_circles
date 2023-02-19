use nannou::prelude::*;
mod ball;
mod boundary;
mod partition;
mod solver;
mod spawn;
use boundary::*;
use nannou::image::io::Reader;
use nannou::image::DynamicImage;
use nannou::ui::color::DARK_CHARCOAL;
use partition::*;
use solver::*;
use spawn::*;
use std::fs;
use std::io::Cursor;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    solver: Solver<RectBound>,
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
    let ball_radius = 5.;
    Model {
        ball_radius,
        frames_for_color_reset: 2000,
        boundary_time: 0.,
        sync_frames: 0,
        sim_runs: 0,
        color_image: Reader::open("cat.jpg").unwrap().decode().unwrap(),
        spawners: vec![
            LinearSpawner::new(
                Vec2::new(-400., 400.),
                0.7 * -PI / 4.,
                2,
                5.,
                10,
                false,
                9530,
            ),
            // LinearSpawner::new(Vec2::new(150., 200.), PI, 3, 5., 10, false, 6074),
            // LinearSpawner::new(Vec2::new(-150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(150., -150.), 0., 6, 4., 5, false),
            // LinearSpawner::new(Vec2::new(-150., 150.), 0., 6, 4., 5, false),
        ],
        solver: Solver {
            gravity: Vec2::new(0.0, -20000.),
            balls: Solver::<CircleBound>::init_balls(ball_radius),
            hash: SpatialHash::new(ball_radius, 1000., 1000.),
            substeps: 10,
            pixel_scale: 1000. / 250.,
            detect_mode: DetectMode::SpatialPartition,
            colormap: vec![],
            boundaries: vec![
                // CircleBound {
                //     pos: Vec2::new(0., 0.),
                //     radius: 400.,
                //     kind: BoundaryType::Inner,
                // },
                // CircleBound {
                //     pos: Vec2::new(400., 0.),
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
                RectBound {
                    pos: Vec2::new(0., 0.),
                    kind: BoundaryType::Inner,
                    width: 900.,
                    height: 900.,
                },
                // RectBound {
                //     pos: Vec2::new(0., 0.),
                //     kind: BoundaryType::Outer,
                //     width: 300.,
                //     height: 100.,
                // },
            ],
        },

        timestep: 0.008,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Critical Updates
    _model.solver.update(_model.timestep);
    _model.boundary_time += _model.timestep;
    let frames = _app.elapsed_frames() as usize;
    if _model.sim_runs < 2 {
        _model.sync_frames += 1;
    }
    // println!("Frame {}", frames);

    // Animations
    let w = -2. * 3.14159 / 10.;
    let r = 400. - 20.;
    // let mouse_bound = &mut _model.solver.boundaries[1];
    // mouse_bound.pos.x = _app.mouse.x;
    // mouse_bound.pos.y = _app.mouse.y;
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
            |t| 0.,
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
