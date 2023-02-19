use grid::*;
use nannou::prelude::*;

pub struct SpatialHash {
    pub grid: Grid<Vec<usize>>,
    pub resolution: f32,
    win_width: f32,
    win_height: f32,
}

impl SpatialHash {
    pub fn new(radius: f32, win_width: f32, win_height: f32) -> Self {
        let res = 2. * radius;
        let nrow = (win_height / res) as usize + 1;
        let ncol = (win_width / res) as usize + 1;
        let grid = Grid::new(ncol, nrow);
        SpatialHash {
            grid,
            resolution: res,
            win_height,
            win_width,
        }
    }

    pub fn hash(&mut self, pos: Vec2, index: usize) {
        // println!("{:?}", pos);
        let mut px = 0.;
        let mut py = 0.;
        py = ((self.win_height / 2.0) - pos.y) / self.resolution;
        px = ((self.win_width / 2.0) + pos.x) / self.resolution;
        // println!("{} {}", px, py);
        let list = self.grid.get_mut(py as usize, px as usize);
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
                                (cc as f32 * self.resolution) - self.win_width / 2.0,
                                (-1. * cr as f32 * self.resolution) + self.win_height / 2.0,
                            ))
                            .wh(Vec2::new(self.resolution, self.resolution))
                            .stroke(WHITE)
                            .stroke_weight(0.5)
                            .rgba(1., 0., 0., 0.1);
                    } else {
                        draw.rect()
                            .xy(Vec2::new(
                                (cc as f32 * self.resolution) - self.win_width / 2.0,
                                (-1. * cr as f32 * self.resolution) + self.win_height / 2.0,
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
