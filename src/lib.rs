extern crate piston_window;
extern crate nalgebra as na;
extern crate ncollide as nc;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;

mod renderable;
mod movable;
use renderable::*;
use movable:: *;

mod util;

mod square_block;

mod tetromino;
use tetromino::*;

use piston_window::*;

mod type_aliases {
    pub use ::na::*;
    pub type Vec2 = Vector2<f64>;
    pub type Point = Point2<f64>;
}

mod transformations;

const BLOCK_SIZE: f64 = 40.0;

use nc::shape::Cuboid;
use nc::shape::Cuboid2;
    
use nc::bounding_volume::HasBoundingVolume;
use nc::bounding_volume::BoundingVolume;
use nc::bounding_volume::AABB;

use std::f64::consts::FRAC_PI_2;

use type_aliases::*;

struct Game {
    tetromino: Tetromino,
    board_cuboid: Cuboid2<f64>
}

impl Game {
    fn new() -> Game {
        Game {
            tetromino: Tetromino::new(TetrominoShape::T, Direction::LeftToRight),
            board_cuboid: Cuboid::new(Vec2::new(200.0, 200.0))
        }
    }

    fn on_load(&mut self, _w: &PistonWindow) {
        // TODO load and assign textures etc.
    }

    fn on_draw<E: GenericEvent>(&mut self, e: &E, w: &mut PistonWindow) {
        w.draw_2d(e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // BOARD
            let board_bv: AABB<Point2<f64>> = self.board_cuboid.bounding_volume(
                &::na::Isometry2::new(
                    Vec2::new(200.0, 240.0), 
                    ::na::zero()
                )
            );
            let board = [
                board_bv.mins().x, 
                board_bv.mins().y, 
                board_bv.half_extents().x * 2.0, 
                board_bv.half_extents().y * 2.0
            ];
            rectangle(
                [0.1, 0.1, 0.1, 1.0],
                board,
                c.transform,
                g
            );

            // SHAPE
            let state = &self.tetromino.state;
            let shape_bv: AABB<Point2<f64>> = state.shape.bounding_volume(&state.isometry);
            let shape = [
                shape_bv.mins().x, 
                shape_bv.mins().y, 
                shape_bv.half_extents().x * 2.0, 
                shape_bv.half_extents().y * 2.0
            ];

            rectangle(
                [0.1, 0.3, 0.1, 0.5],
                shape,
                c.transform,
                g
            );

            let r_point = self.tetromino.state.rotation_point;
            self.tetromino.render(g, c.transform.trans(300.0 - r_point.x, 300.0 - r_point.y));
        });
    }

    fn on_update(&mut self, upd: &UpdateArgs) {
        self.tetromino.update(upd.dt);
    }

    fn on_input<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            let tetromino = &mut self.tetromino;
            let transformation: Isometry2<f64> = match key {
                Key::Up     => Isometry2::new(Vec2::new(0.0, -BLOCK_SIZE), zero()),
                Key::Down   => Isometry2::new(Vec2::new(0.0,  BLOCK_SIZE), zero()),
                Key::Right  => Isometry2::new(zero(),                      FRAC_PI_2),
                Key::Left   => Isometry2::new(zero(),                     -FRAC_PI_2),
                _           => Isometry2::identity()
            };

            let correction: Isometry2<f64> = match Game::check_board_collision(tetromino, &self.board_cuboid, &transformation) {
                Collision::None     => Isometry2::identity(),
                Collision::Top      => Isometry2::new(Vec2::new(0.0,  BLOCK_SIZE), zero()),
                Collision::Bottom   => Isometry2::new(Vec2::new(0.0, -BLOCK_SIZE), zero()),
            };

            tetromino.apply(util::isometry::add_isometries(&transformation, &correction));
        };
    }

    fn check_board_collision(tetromino: &Tetromino, board_cuboid: &Cuboid2<f64>, transformation: &Isometry2<f64>) -> Collision {
        let board_bv: AABB<Point2<f64>> = board_cuboid.bounding_volume(
            &::na::Isometry2::new(
                Vec2::new(200.0, 240.0), 
                ::na::zero()
            )
        );
        let loosened_board_bv = &board_bv.loosened(BLOCK_SIZE);
        let shape_bv: AABB<Point2<f64>> = tetromino.state.shape.bounding_volume(&util::isometry::add_isometries(&tetromino.state.isometry, &transformation));

        let contains = loosened_board_bv.contains(&shape_bv);

        if contains {
            Collision::None
        } else {
            if shape_bv.mins().y.round() < board_bv.mins().y.round() {
                Collision::Top
            } else {
                Collision::Bottom
            }
        }
    }

}

enum Collision {
    None,
    Top,
    Bottom
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