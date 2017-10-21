extern crate piston_window;
extern crate nalgebra as na;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

mod square;
use square::*;

use piston_window::*;

type Vec2 = na::Vector2<f64>;

struct Game {
    tetromino: Tetromino,
    up_pressed: bool,
    down_pressed: bool,
    scx: f64,
    scy: f64
}

impl Game {
    fn new() -> Game {
        Game {
            tetromino: Tetromino::new_s(),
            scx: 300.0,
            scy: 300.0,
            up_pressed: false,
            down_pressed: false
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
        if self.up_pressed {
            self.tetromino.mov(Vec2::new(0.0, -10.0));
        }
        if self.down_pressed {
            self.tetromino.mov(Vec2::new(0.0, 10.0));
        }
    }

    fn on_input<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Up => self.up_pressed = true,
                Key::Down => self.down_pressed = true,
                Key::Return => self.tetromino.rot(45.0),
                _ => {}
            }
        };
        if let Some(Button::Keyboard(key)) = e.release_args() {
            match key {
                Key::Up => self.up_pressed = false,
                Key::Down => self.down_pressed = false,
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