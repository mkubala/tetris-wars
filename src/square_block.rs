use ::piston_window::*;
use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use type_aliases::*;
use transformations::Transformation;
use movable::Movable;
use renderable::Renderable;

pub struct SquareBlock {
    offset: Vec2,
    trans: Transformation,
    sprite: Option<Texture<Resources>>
}

impl SquareBlock {
    pub fn new(offset: Vec2) -> SquareBlock {
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