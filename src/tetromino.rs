use ::piston_window::*;

use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use type_aliases::*;

use square_block::*;

use renderable::Renderable;
use movable::Movable;

pub struct Tetromino {
    blocks: [SquareBlock; 4]
}

impl Tetromino {
    pub fn new_s() -> Tetromino {
        // TODO this will depend on the rotation direction.
        let logical_center = Vec2::new(60.0, 40.0);
        Tetromino {
            blocks: [
                SquareBlock::new(Vec2::new(0.0, 0.0) - logical_center),
                SquareBlock::new(Vec2::new(40.0, 0.0) - logical_center),
                SquareBlock::new(Vec2::new(40.0, 40.0) - logical_center),
                SquareBlock::new(Vec2::new(80.0, 40.0) - logical_center)
            ]        
        }
    }
}

impl Movable for Tetromino {

    fn mov_up(&mut self, dy: f64) {
        for block in &mut self.blocks {
            block.mov_up(dy);
        }
    }
    
    fn mov_down(&mut self, dy: f64) {
        for block in &mut self.blocks {
            block.mov_down(dy);
        }
    }

    fn rot_left(&mut self) {
        for block in &mut self.blocks {
            block.rot_left();
        }
    }

    fn rot_right(&mut self) {
        for block in &mut self.blocks {
            block.rot_right();
        }
    }

    fn update(&mut self, dt: f64) {
        for block in &mut self.blocks {
            block.update(dt);
        }
    }
}

impl Renderable for Tetromino {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        for block in &self.blocks {
            block.render(g, view);
        }
    }
}