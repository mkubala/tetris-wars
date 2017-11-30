use ::piston_window::*;

use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use type_aliases::*;

use square_block::*;
use ::util::isometry::*;

use renderable::Renderable;
use movable::Movable;

use ::na::zero;
use ::na::Isometry2;
use ::nc::shape::{ Cuboid, Compound2 };
use ::nc::shape::ShapeHandle;

use std::f64::consts::FRAC_PI_2;

use BLOCK_SIZE;

pub enum Direction {
    LeftToRight,
    RightToLeft
}

pub enum TetrominoShape {
    Z,
    S,
    O,
    L,
    J,
    I,
    T
}

type Points = [Point; 4];

impl TetrominoShape {

    fn initial_state(&self) -> (Points, TetrominoState) {
        let blocks_pos = match self {
            &TetrominoShape::Z  => [Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0)],
            &TetrominoShape::S  => [Point::new(1.0, 0.0), Point::new(2.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 1.0)],
            &TetrominoShape::O  => [Point::new(1.0, 0.0), Point::new(2.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0)],
            &TetrominoShape::L  => [Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(1.0, 2.0), Point::new(2.0, 2.0)],
            &TetrominoShape::J  => [Point::new(1.0, 0.0), Point::new(1.0, 1.0), Point::new(1.0, 2.0), Point::new(0.0, 2.0)],
            &TetrominoShape::I  => [Point::new(0.0, 1.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0), Point::new(3.0, 1.0)],
            &TetrominoShape::T  => [Point::new(1.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 1.0), Point::new(2.0, 1.0)]
        };
        let rotation_point = match self {
            &TetrominoShape::O  => Point::new(1.5 * BLOCK_SIZE, 0.5 * BLOCK_SIZE),
            &TetrominoShape::I  => Point::new(1.5 * BLOCK_SIZE, 1.5 * BLOCK_SIZE),
            _                   => Point::new(1.0 * BLOCK_SIZE, 1.0 * BLOCK_SIZE)
        };
        
        let cuboid_handle = ShapeHandle::new(Cuboid::new(Vec2::new(BLOCK_SIZE / 2.0, BLOCK_SIZE / 2.0)));
        let compound_shape = Compound2::new(
            blocks_pos
                .iter()
                .map(|p| {
                    let transition = Vec2::new(p.x * BLOCK_SIZE, p.y * BLOCK_SIZE) - Vec2::new(rotation_point.x, rotation_point.y); // - Vec2::new(0.5 * BLOCK_SIZE, 0.5 * BLOCK_SIZE);
                    (
                        Isometry2::new(transition, zero()), 
                        cuboid_handle.clone()
                    )
                })
                .collect()
        );
        let state = TetrominoState {
            shape: compound_shape,
            isometry: Isometry2::new(Vec2::new(300.0 - rotation_point.x, 300.0 - rotation_point.y), zero()),
            rotation_point
        };
        (blocks_pos, state)
    }

}

pub struct TetrominoState {
    pub shape: Compound2<f64>,
    pub isometry: Isometry2<f64>,
    pub rotation_point: Point
}

pub struct Tetromino {
    pub state: TetrominoState,
    blocks: Vec<SquareBlock>
}

// This implementation obeys Super Rotation System (http://tetris.wikia.com/wiki/SRS)
impl Tetromino {

    // TODO We will either flip or transpose tetromino coords, depending on the direction 
    // (left to right = transpose; right to left = rotate)
    pub fn new(shape: TetrominoShape, dir: Direction) -> Tetromino {
        let (points, state) = shape.initial_state();
        let blocks: Vec<SquareBlock> = points.iter()
            .map(|point| {
                let v = Vector2::new(point.x * BLOCK_SIZE, point.y * BLOCK_SIZE) - Vec2::new(state.rotation_point.x, state.rotation_point.y) - Vector2::new(0.5 * BLOCK_SIZE, 0.5 * BLOCK_SIZE);
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
    }
    
    fn mov_down(&mut self) {
        self.state.mov_down();
        for block in &mut self.blocks {
            block.mov_down();
        }
    }

    fn rot_left(&mut self) {
        self.state.rot_left();
        for block in &mut self.blocks {
            block.rot_left();
        }
    }

    fn rot_right(&mut self) {
        self.state.rot_right();
        for block in &mut self.blocks {
            block.rot_right();
        }
    }

    fn update(&mut self, dt: f64) {
        for block in &mut self.blocks {
            block.update(dt);
        }
    }

    fn apply(&mut self, iso: Isometry2<f64>) {
        self.state.apply(iso);
        for block in &mut self.blocks {
            block.apply(iso)
        }
    }
}

impl Movable for TetrominoState {

    fn mov_up(&mut self) {
        self.apply(Isometry2::new(Vec2::new(0.0, -BLOCK_SIZE), zero()));
    }
    
    fn mov_down(&mut self) {
        self.apply(Isometry2::new(Vec2::new(0.0, BLOCK_SIZE), zero()));
    }

    fn rot_left(&mut self) {
        let iso = Isometry2::new(
                zero(), 
                -FRAC_PI_2
            );
        self.apply(iso);
    }

    fn rot_right(&mut self) {
        let iso = Isometry2::new(
                zero(), 
                FRAC_PI_2
            );
        self.apply(iso);
    }

    fn update(&mut self, dt: f64) {}

    fn apply(&mut self, iso: Isometry2<f64>) {
        self.isometry = add_isometries(&self.isometry, &iso);
    }
}

impl Renderable for Tetromino {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        for block in &self.blocks {
            block.render(g, view);
        }
    }
}