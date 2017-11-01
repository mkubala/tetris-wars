extern crate piston_window;
extern crate nalgebra as na;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

mod renderable;
mod movable;
use renderable::*;
use movable:: *;

mod square_block;

mod tetromino;
use tetromino::*;

use piston_window::*;

mod type_aliases {
    pub use ::na::*;
    pub type Vec2 = Vector2<f64>;
    pub type Vec3 = Vector3<f64>;
    pub type Point = Point2<f64>;
}

mod transformations;

const BLOCK_SIZE: f64 = 40.0;

struct Game {
    tetromino: Tetromino,
    scx: f64,
    scy: f64
}

impl Game {
    fn new() -> Game {
        Game {
            tetromino: Tetromino::new(TetrominoShape::L, Direction::LeftToRight),
            scx: BLOCK_SIZE * 7.5,
            scy: BLOCK_SIZE * 7.5
        }
    }

    fn on_load(&mut self, w: &PistonWindow) {
        // TODO load and assign textures etc.
    }

    fn on_draw<E: GenericEvent>(&mut self, e: &E, w: &mut PistonWindow) {
        let size = w.size();
        self.scx = (size.width / 2) as f64;
        self.scy = (size.height / 2) as f64;
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans(self.scx, self.scy);
            self.tetromino.render(g, center);
        });
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
        self.tetromino.update(upd.dt);
    }

    fn on_input<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => self.tetromino.mov_up(),
                Key::Down => self.tetromino.mov_down(),
                Key::Right => self.tetromino.rot_right(),
                Key::Left => self.tetromino.rot_left(),
                _ => {}
            }
        };
    }
}

pub fn run() {
    let mut window: PistonWindow = WindowSettings::new(
        "tetris-wars",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new();
    
    game.on_load(&window);
    
    while let Some(e) = window.next() {
        game.on_input(&e);
        if let Some(upd) = e.update_args() {
            game.on_update(&upd);
        }
        game.on_draw(&e, &mut window);
    }

}