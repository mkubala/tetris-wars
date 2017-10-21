use ::piston_window::*;

use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use ::na::{Vector2, Vector3};

type Vec2 = Vector2<f64>;
type Vec3 = Vector3<f64>;

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
    trans: Transform,
    sprite: Option<Texture<Resources>>
}

impl SquareBlock {
    fn new(offset: Vec2) -> SquareBlock {
        SquareBlock {
            offset,
            trans: Transform::new(),
            sprite: Option::None
        }
    }

}

#[derive(Copy, Clone)]
pub struct Transform {
    pos_rot: Vec3,
    target: Vec3,
    step: Vec3
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            pos_rot: Vec3::zeros(),
            target: Vec3::zeros(),
            step: Vec3::zeros()
        }
    }

    fn update_step(&mut self) {
        self.step = (self.target - self.pos_rot) / 10.0;
        println!("new step = {}", self.step);
    }

}

pub trait Movable {
    fn mov(&mut self, pos: Vec2);
    fn rot(&mut self, r: f64);
    fn update(&mut self, dt: f64);
}

impl Movable for Transform {
    fn mov(&mut self, v: Vec2) {
        let x: Vec3 = Vec3::new(v.x, v.y, 0.0);
        self.target = self.target + x;
        self.update_step();
    }

    fn rot(&mut self, d: f64) {
        let mut x = Vec3::zeros();
        x.z = d;
        self.target = self.target + x;
        self.update_step();
    }

    fn update(&mut self, dt: f64) {
        if self.step != Vec3::zeros() && (self.pos_rot - self.target).abs() <= Vec3::new(1.0, 1.0, 1.0) {
            self.step = Vec3::zeros();
            self.pos_rot = self.target;
        } else {
            self.pos_rot = self.pos_rot + self.step;
        }
    }
}

impl Movable for SquareBlock {

    fn mov(&mut self, pos: Vec2) {
        self.trans.mov(pos);
    }
    
    fn rot(&mut self, r: f64) {
        self.trans.rot(r);
    }
    
    fn update(&mut self, dt: f64) {
        self.trans.update(dt);
    }
}

impl Movable for Tetromino {

    fn mov(&mut self, pos: Vec2) {
        for block in &mut self.blocks {
            block.mov(pos);
        }
    }

    fn rot(&mut self, r: f64) {
        for block in &mut self.blocks {
            block.rot(r);
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
        let t: Transform = self.trans;
        let o = self.offset;
        let tile_size = 40.0;
        let square = rectangle::square(0.0, 0.0, tile_size);
        let transition = view.trans(t.pos_rot.x, t.pos_rot.y)
                             .rot_deg(t.pos_rot.z)
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