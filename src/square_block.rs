use ::piston_window::*;
use ::gfx_device_gl::{Resources, CommandBuffer};
use ::gfx_graphics::GfxGraphics;

use type_aliases::*;
use transformations::Transformation;
use movable::Movable;
use renderable::Renderable;

pub struct SquareBlock {
    local_coords: Vec2,
    trans: Transformation//,
    // sprite: Option<Texture<Resources>>
}

impl SquareBlock {
    pub fn new(local_coords: Vec2) -> SquareBlock {
        SquareBlock {
            local_coords,
            trans: Transformation::empty(),
            // sprite: Option::None
        }
    }

    fn transform(&mut self, transformation: Transformation) {
        self.trans = self.trans + transformation;
    }

}

impl Movable for SquareBlock {
    
    fn update(&mut self, dt: f64) {
        self.trans.update(dt);
    }

    fn apply(&mut self, iso: Isometry2<f64>) {
        self.transform(Transformation::new(iso));
    }
}

impl Renderable for SquareBlock {
    fn render(&self, g: &mut GfxGraphics<Resources, CommandBuffer>, view: math::Matrix2d) {
        let t: Transformation = self.trans;
        let local = self.local_coords;
        let square = rectangle::square(0.0, 0.0, ::BLOCK_SIZE);
        let transition = view.trans(t.x(), t.y())
                             .rot_rad(t.rot_rad())
                             .trans(local.x, local.y);
        rectangle(
            [1.0, 0.0, 0.0, 1.0], 
            square, 
            transition, 
            g);
    }
}