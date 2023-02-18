use nannou::prelude::*;
mod ball;
mod boundary;
mod solver;
use boundary::*;
use solver::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    solver: Solver<CircleBound>,
    timestep: f32,
}

fn model(_app: &App) -> Model {
    Model {
        solver: Solver {
            gravity: Vec2::new(0.0, -2000.),
            balls: Solver::<CircleBound>::init_balls(3.),
            hash: SpatialHash::new(3., 1500., 1500.),
            substeps: 8,
            boundaries: vec![
                CircleBound {
                    pos: Vec2::new(0., 0.),
                    radius: 400.,
                    kind: BoundaryType::Inner,
                },
                CircleBound {
                    pos: Vec2::new(2000., 2000.),
                    radius: 70.,
                    kind: BoundaryType::Outer,
                },
                // CircleBound {
                //     pos: Vec2::new(-125., -450.),
                //     radius: 80.,
                //     kind: BoundaryType::Outer,
                // },
                // CircleBound {
                //     pos: Vec2::new(0., 0.),
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
    let w = 2. * 3.14159 / 40.;
    let time = (_app.elapsed_frames() as f32) * _model.timestep;
    // let r = 50. + (200. * (time * 5. * w).sin());
    let r = 400. - 20.;
    // let mouse_bound = &mut _model.solver.boundaries[1];
    // mouse_bound.pos.x = _app.mouse.x;
    // mouse_bound.pos.y = _app.mouse.y;
    let move_bound = &mut _model.solver.boundaries[1];
    move_bound.pos.x = r * (time * w).cos();
    move_bound.pos.y = r * (time * w).sin();
    // let first_bound = &mut _model.solver.boundaries[0];
    // first_bound.radius = r + 100. * (time * w).sin(1;

    // let inner_bound = &mut _model.solver.boundaries[3];
    // inner_bound.radius = 60. + 20. * (time * 8. * w).sin();
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
