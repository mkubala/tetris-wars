use ::piston_window::*;

use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use type_aliases::*;

use square_block::*;

use renderable::Renderable;
use movable::Movable;

pub enum Direction {
    LeftToRight,
    RightToLeft
}

pub enum TetrominoShape {
    Z,
    S,
    O,
    L,
    J
}

type Points = [Point; 4];

impl TetrominoShape {

    fn initial_state(&self) -> TetrominoState {
        let blocks_pos = match self {
            &TetrominoShape::Z  => [Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0)],
            &TetrominoShape::S  => [Point::new(1.0, 0.0), Point::new(2.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 1.0)],
            &TetrominoShape::O  => [Point::new(1.0, 0.0), Point::new(2.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0)],
            &TetrominoShape::L  => [Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(1.0, 2.0), Point::new(2.0, 2.0)],
            &TetrominoShape::J  => [Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(1.0, 2.0), Point::new(0.0, 2.0)]
        };
        let rotation_point = match self {
            &TetrominoShape::O  => Vector2::new(1.5, 0.5),
            _                   => Vector2::new(1.0, 1.0)
        };
        TetrominoState {
            blocks_pos,
            rotation_point
        }
    }

}

struct TetrominoState {
    blocks_pos: Points,
    rotation_point: Vector2<f64>
}

pub struct Tetromino {
    state: TetrominoState,
    blocks: Vec<SquareBlock>
}

// This implementation obeys Super Rotation System (http://tetris.wikia.com/wiki/SRS)
impl Tetromino {

    // TODO We will either flip or transpose tetromino coords, depending on the direction 
    // (left to right = transpose; right to left = rotate)
    pub fn new(shape: TetrominoShape, dir: Direction) -> Tetromino {
        let state = shape.initial_state();
        let blocks: Vec<SquareBlock> = state.blocks_pos.iter()
            .map(|point| {
                let v = Vector2::new(point.x as f64, point.y as f64) - (state.rotation_point + Vector2::new(0.5, 0.5));
                SquareBlock::new(v)
            })
            .collect();
        Tetromino {
            state,
            blocks
        }
    }

}

impl Movable for Tetromino {

    fn mov_up(&mut self) {
        self.state.mov_up();
        for block in &mut self.blocks {
            block.mov_up();
        }
        println!("points after move: {:?}", &self.state.blocks_pos);
    }
    
    fn mov_down(&mut self) {
        self.state.mov_down();
        for block in &mut self.blocks {
            block.mov_down();
        }
        println!("points after move: {:?}", &self.state.blocks_pos);
    }

    fn rot_left(&mut self) {
        self.state.rot_left();
        for block in &mut self.blocks {
            block.rot_left();
        }
        println!("points after rotation: {:?}", &self.state.blocks_pos);
    }

    fn rot_right(&mut self) {
        self.state.rot_right();
        for block in &mut self.blocks {
            block.rot_right();
        }
        println!("points after rotation: {:?}", &self.state.blocks_pos);
    }

    fn update(&mut self, dt: f64) {
        for block in &mut self.blocks {
            block.update(dt);
        }
    }
}

impl Movable for TetrominoState {

    fn mov_up(&mut self) {
        let d = Vector2::new(0.0, -1.0);
        self.blocks_pos = [
            self.blocks_pos[0] + d,
            self.blocks_pos[1] + d,
            self.blocks_pos[2] + d,
            self.blocks_pos[3] + d
        ];
    }
    
    fn mov_down(&mut self) {
        let d = Vector2::new(0.0, 1.0);
        self.blocks_pos = [
            self.blocks_pos[0] + d,
            self.blocks_pos[1] + d,
            self.blocks_pos[2] + d,
            self.blocks_pos[3] + d
        ];
    }

    fn rot_left(&mut self) {
        // 90 degrees counter-clockwise
        // 0  1
        // -1 0
        let rot = ::na::Matrix2::new(0.0, 1.0, -1.0, 0.0);
        let t = self.rotation_point;
        self.blocks_pos = [
            rot * (self.blocks_pos[0] - t) + t,
            rot * (self.blocks_pos[1] - t) + t,
            rot * (self.blocks_pos[2] - t) + t,
            rot * (self.blocks_pos[3] - t) + t
        ];
    }

    fn rot_right(&mut self) {
        // 90 degrees clockwise
        // 0 -1
        // 1  0
        let rot = ::na::Matrix2::new(0.0, -1.0, 1.0, 0.0);
        let t = self.rotation_point;
        self.blocks_pos = [
            rot * (self.blocks_pos[0] - t) + t,
            rot * (self.blocks_pos[1] - t) + t,
            rot * (self.blocks_pos[2] - t) + t,
            rot * (self.blocks_pos[3] - t) + t
        ];
    }

    fn update(&mut self, dt: f64) {}
}

impl Renderable for Tetromino {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        for block in &self.blocks {
            block.render(g, view);
        }
    }
}