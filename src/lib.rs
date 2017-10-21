extern crate piston_window;
extern crate nalgebra as na;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

mod square;
use square::*;

use piston_window::*;

mod type_aliases {
    pub type Vec2 = ::na::Vector2<f64>;
    pub type Vec3 = ::na::Vector3<f64>;
}
use type_aliases::*;

mod transformations;

struct Game {
    tetromino: Tetromino,
    scx: f64,
    scy: f64
}

impl Game {
    fn new() -> Game {
        Game {
            tetromino: Tetromino::new_s(),
            scx: 300.0,
            scy: 300.0
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
                Key::Up => self.tetromino.mov(Vec2::new(0.0, -40.0)),
                Key::Down => self.tetromino.mov(Vec2::new(0.0, 40.0)),
                Key::Right => self.tetromino.rot(90.0),
                Key::Left => self.tetromino.rot(-90.0),
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