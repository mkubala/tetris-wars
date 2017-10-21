use ::piston_window::*;

use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use ::na::{Vector2, Vector3};

use type_aliases::*;

use transformations::*;

pub struct Tetromino {
    pos: Vec2,
    blocks: [SquareBlock; 4]
}

impl Tetromino {
    pub fn new_s() -> Tetromino {
        // TODO this will depend on the rotation direction.
        let logical_center = Vec2::new(60.0, 40.0);
        Tetromino {
            pos: Vec2::new(0.0, 0.0),
            blocks: [
                SquareBlock::new(Vec2::new(0.0, 0.0) - logical_center),
                SquareBlock::new(Vec2::new(40.0, 0.0) - logical_center),
                SquareBlock::new(Vec2::new(40.0, 40.0) - logical_center),
                SquareBlock::new(Vec2::new(80.0, 40.0) - logical_center)
            ]        
        }
    }
}

pub struct SquareBlock {
    offset: Vec2,
    trans: Transformation,
    sprite: Option<Texture<Resources>>
}

impl SquareBlock {
    fn new(offset: Vec2) -> SquareBlock {
        SquareBlock {
            offset,
            trans: Transformation::empty(),
            sprite: Option::None
        }
    }

    fn transform(&mut self, transformation: Transformation) {
        self.trans = self.trans + transformation;
    }

}

pub trait Movable {
    fn mov_up(&mut self, dy: f64);
    fn mov_down(&mut self, dy: f64);
    fn rot_left(&mut self);
    fn rot_right(&mut self);
    fn update(&mut self, dt: f64);
}

impl Movable for SquareBlock {

    fn mov_up(&mut self, dy: f64) {
        self.transform(Transformation::move_up(dy));
    }

    fn mov_down(&mut self, dy: f64) {
        self.transform(Transformation::move_down(dy));
    }

    fn rot_left(&mut self) {
        self.transform(Transformation::rot_left(90.0));
    }

    fn rot_right(&mut self) {
        self.transform(Transformation::rot_right(90.0));
    }
    
    fn update(&mut self, dt: f64) {
        self.trans.update(dt);
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

pub trait Renderable {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d);
}

impl Renderable for SquareBlock {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let t: Transformation = self.trans;
        let o = self.offset;
        let tile_size = 40.0;
        let square = rectangle::square(0.0, 0.0, tile_size);
        let transition = view.trans(t.x(), t.y())
                             .rot_deg(t.rot())
                             .trans(o.x, o.y);
        rectangle(
            [1.0, 0.0, 0.0, 1.0], 
            square, 
            transition, 
            g);
    }
}

impl Renderable for Tetromino {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        for block in &self.blocks {
            block.render(g, view);
        }
    }
}