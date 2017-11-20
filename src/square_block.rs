use ::piston_window::*;
use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use type_aliases::*;
use transformations::Transformation;
use movable::Movable;
use renderable::Renderable;

pub struct SquareBlock {
    local_coords: Vec2,
    trans: Transformation,
    sprite: Option<Texture<Resources>>
}

impl SquareBlock {
    pub fn new(local_coords: Vec2) -> SquareBlock {
        SquareBlock {
            local_coords,
            trans: Transformation::empty(),
            sprite: Option::None
        }
    }

    fn transform(&mut self, transformation: Transformation) {
        self.trans = self.trans + transformation;
    }

}

impl Movable for SquareBlock {

    fn mov_up(&mut self) {
        self.transform(Transformation::move_up(::BLOCK_SIZE));
    }

    fn mov_down(&mut self) {
        self.transform(Transformation::move_down(::BLOCK_SIZE));
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

impl Renderable for SquareBlock {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let t: Transformation = self.trans;
        let local = self.local_coords;
        let square = rectangle::square(0.0, 0.0, ::BLOCK_SIZE);
        let transition = view.trans(t.x(), t.y())
                             .rot_deg(t.rot())
                             .trans(local.x, local.y);
        rectangle(
            [1.0, 0.0, 0.0, 1.0], 
            square, 
            transition, 
            g);
    }
}