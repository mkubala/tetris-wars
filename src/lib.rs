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

use util::isometry::*;

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

            // CUBE
            let cube = Cuboid2::new(Vec2::new(20.0, 20.0));
            let cube_iso = Isometry2::new(Vec2::new(220.0, 100.0), zero());
            let cube_bv: AABB<Point2<f64>> = cube.bounding_volume(&cube_iso);
            let cube_shape = [
                cube_bv.mins().x, 
                cube_bv.mins().y, 
                cube_bv.half_extents().x * 2.0, 
                cube_bv.half_extents().y * 2.0
            ];

            rectangle(
                [0.0, 0.0, 0.8, 0.3],
                cube_shape,
                c.transform,
                g
            );

            // SHAPE
            &self.tetromino.state.shape.shapes()
                .iter()
                .for_each(|&(iso, ref shape_handle)| {
                    let mut effective_iso: Isometry2<f64> = add_isometries(&iso, &self.tetromino.state.isometry);
                    effective_iso.append_rotation_wrt_point_mut(
                        &self.tetromino.state.isometry.rotation,
                        &(::na::Point::origin() + self.tetromino.state.isometry.translation.vector)
                    );

                    let shape_bv: AABB<Point2<f64>> = shape_handle.bounding_volume(&effective_iso);
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
                });

            let shape_bv: AABB<Point2<f64>> = self.tetromino.state.shape.bounding_volume(&self.tetromino.state.isometry);
            let shape = [
                    shape_bv.mins().x, 
                    shape_bv.mins().y, 
                    shape_bv.half_extents().x * 2.0, 
                    shape_bv.half_extents().y * 2.0
                ];

                rectangle(
                    [0.0, 0.4, 0.0, 0.2],
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
                Collision::None     => {
                    println!("Collision: NONE");
                    Isometry2::identity()
                },
                Collision::Top      => {
                    println!("Collision: TOP");
                    Isometry2::new(Vec2::new(0.0,  BLOCK_SIZE), zero())
                },
                Collision::Bottom   => {
                    println!("Collision: BOTTOM");
                    Isometry2::new(Vec2::new(0.0, -BLOCK_SIZE), zero())
                },
            };

            let effective_transformation = add_isometries(&transformation, &correction);

            let cube = Cuboid2::new(Vec2::new(20.0, 20.0));
            let cube_iso = Isometry2::new(Vec2::new(220.0, 100.0), zero());
            let intersects = Game::collides(&tetromino, &cube.bounding_volume(&cube_iso), &effective_transformation);
            println!("intersects? {}", intersects);

            tetromino.apply(effective_transformation);
        };
    }

    fn check_board_collision(tetromino: &Tetromino, board_cuboid: &Cuboid2<f64>, transformation: &Isometry2<f64>) -> Collision {
        let mut board_bv: AABB<Point2<f64>> = board_cuboid.bounding_volume(
            &::na::Isometry2::new(
                Vec2::new(200.0, 240.0), 
                ::na::zero()
            )
        );
        board_bv.loosen(BLOCK_SIZE);
        let transformed_iso = add_isometries(&tetromino.state.isometry, &transformation);
        let shape_bv: AABB<Point2<f64>> = tetromino.state.shape.bounding_volume(&transformed_iso);
        if shape_bv.mins().y < board_bv.mins().y {
            println!("Collision: TOP");
            Collision::Top
        } else if shape_bv.maxs().y > board_bv.maxs().y {
            println!("Collision: Bottom");
            Collision::Bottom
        } else {
            println!("Collision: NONE");
            Collision::None
        }
    }

    fn collides(tetromino: &Tetromino, other_bv: &AABB<Point2<f64>>, transformation: &Isometry2<f64>) -> bool {
        let loosened_oth_bv = other_bv.loosened(1.0);
        tetromino.state.shape.shapes()
            .iter()
            .any(|&(iso, ref shape_handle)| {
                let mut effective_iso = add_isometries(&add_isometries(&tetromino.state.isometry, &transformation), &iso);
                effective_iso.append_rotation_wrt_point_mut(
                    &tetromino.state.isometry.rotation,
                    &(Point::origin() + tetromino.state.isometry.translation.vector)
                );
                let shape_bv = shape_handle.bounding_volume(&effective_iso);
                println!("{:#?} intersects {:#?} ? {}", loosened_oth_bv, shape_bv, loosened_oth_bv.intersects(&shape_bv));
                loosened_oth_bv.intersects(&shape_bv)
            })
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