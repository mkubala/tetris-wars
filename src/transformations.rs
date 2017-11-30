use super::type_aliases::*;
use ::na::Isometry2;
use std::f64::consts::FRAC_PI_2;

use ::util::isometry::*;

#[derive(Copy, Clone, Debug)]
pub struct Transformation {
    to: Isometry2<f64>,
    step: Isometry2<f64>,
    state: Isometry2<f64>
}

const STEPS: f64 = 20.0;

impl Transformation {

    pub fn empty() -> Transformation {
        Transformation {
            to: Isometry2::identity(),
            step: Isometry2::identity(),
            state: Isometry2::identity()
        }
    }

    pub fn new(to: Isometry2<f64>) -> Transformation {
        Transformation {
            to,
            step: div_isometry(&to, STEPS),
            state: Isometry2::identity()
        }
    }

    pub fn move_up(dy: f64) -> Transformation {
        Transformation::new(Isometry2::new(Vec2::new(0.0, -dy), ::na::zero()))
    }

    pub fn move_down(dy: f64) -> Transformation {
        Transformation::new(Isometry2::new(Vec2::new(0.0, dy), ::na::zero()))
    }

    pub fn rot_left(d: f64) -> Transformation {
        Transformation::new(Isometry2::new(::na::zero(), -d))
    }

    pub fn rot_right(d: f64) -> Transformation {
        Transformation::new(Isometry2::new(::na::zero(), d))
    }

    pub fn x(&self) -> f64 {
        self.state.translation.vector.x
    }

    pub fn y(&self) -> f64 {
        self.state.translation.vector.y
    }

    pub fn rot(&self) -> f64 {
        self.state.rotation.angle()
    }

    // TODO take dt into account
    pub fn update(&mut self, dt: f64) {
        if self.step != Isometry2::identity() {
            let rotation_is_finished: bool = {
                let rot_diff = (self.state.rotation.angle() - self.to.rotation.angle()).abs();
                let rot_eps = FRAC_PI_2 / STEPS;
                rot_diff <= rot_eps
            };

            if rotation_is_finished {
                self.step.rotation = UnitComplex::from_angle(0.0);
                self.state.rotation = self.to.rotation;
            }

            let translation_is_finished: bool = {
                let trans_y_diff = (self.state.translation.vector.y - &self.to.translation.vector.y).abs();
                let trans_eps = ::BLOCK_SIZE / STEPS;
                trans_y_diff <= trans_eps
            };
            
            if translation_is_finished {
                self.step.translation = Translation2::identity();
                self.state.translation = self.to.translation;
            } 
            
            if !(rotation_is_finished && translation_is_finished) {
                self.state = add_isometries(&self.state, &self.step)
            }
        }
    }
}

impl ::std::ops::Add for Transformation {
    type Output = Transformation;

    fn add(self, rhs: Transformation) -> Transformation {
        let new_to = add_isometries(&self.to, &rhs.to);
        let new_trans = Transformation {
            to: new_to,
            step: div_isometry(&sub_isometries(&new_to, &self.state), STEPS),
            state: self.state
        };
        new_trans
    }
}
