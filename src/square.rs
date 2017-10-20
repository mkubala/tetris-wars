use ::piston_window::*;

use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use ::na::Vector2;

type Vec2 = Vector2<f64>;

pub struct Tetromino {
    pos: Vec2,
    blocks: [SquareBlock; 1]
}

impl Tetromino {
    pub fn new() -> Tetromino {
        Tetromino {
            pos: Vec2::new(0.0, 0.0),
            blocks: [
                SquareBlock::new()
            ]
        }
    }
}

pub struct SquareBlock {
    trans: Transform,
    sprite: Option<Texture<Resources>>
}

impl SquareBlock {
    fn new() -> SquareBlock {
        SquareBlock {
            trans: Transform::new(),
            sprite: Option::None
        }
    }

}

#[derive(Copy, Clone)]
pub struct Transform {
    pos: Vec2,
    rot: f64
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            pos: Vec2::new(0.0, 0.0),
            rot: 0.0
        }
    }

    pub fn mov(&mut self, v: Vec2) {
        self.pos = self.pos + v;
    }

    pub fn mov_to(&mut self, v: Vec2) {
        self.pos = v;
    }

    pub fn rot(&mut self, d: f64) {
        self.rot += d;
    }

    pub fn rot_to(&mut self, d: f64) {
        self.rot = d;
    }
}

pub trait Movable {
    fn mov(&mut self, pos: Vec2);
    fn mov_to(&mut self, pos: Vec2);
    fn rot(&mut self, r: f64);
    fn rot_to(&mut self, r: f64);
    fn update(&mut self, dt: f64);
}

impl Movable for SquareBlock {
    fn mov(&mut self, pos: Vec2) {
        self.trans.mov(pos);
    }
    
    fn mov_to(&mut self, pos: Vec2) {}
    fn rot(&mut self, r: f64) {}
    fn rot_to(&mut self, r: f64) {}
    fn update(&mut self, dt: f64) {}
}

impl Movable for Tetromino {
    fn mov(&mut self, pos: Vec2) {
        for block in &mut self.blocks {
            block.mov(pos);
        }
    }
    
    fn mov_to(&mut self, pos: Vec2) {}
    fn rot(&mut self, r: f64) {}
    fn rot_to(&mut self, r: f64) {}
    fn update(&mut self, dt: f64) {}
}

pub trait Renderable {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d);
}

impl Renderable for SquareBlock {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let t: Transform = self.trans;
        let tile_size = 50.0;
        let square = rectangle::square(0.0, 0.0, tile_size);
        let transition = view.trans(t.pos.x, t.pos.y)
                             .rot_rad(t.rot)
                             .trans(-tile_size, -tile_size);
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