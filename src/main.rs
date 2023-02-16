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
            gravity: Vec2::new(0.0, -100.),
            balls: Solver::<CircleBound>::init_balls(15.),
            substeps: 8,
            boundaries: vec![CircleBound {
                pos: Vec2::new(0., 0.),
                radius: 200.,
                kind: BoundaryType::Inner,
            }],
        },
        timestep: 0.01,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.solver.update(_model.timestep);
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    frame.clear(BLACK);

    draw.ellipse().x_y(10., 10.).radius(10.).color(WHITE);
    _model.solver.draw(&draw);
    draw.to_frame(_app, &frame).unwrap();
}
