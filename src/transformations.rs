use super::type_aliases::*;

#[derive(Copy, Clone)]
pub struct Transformation {
    to: Vec3,
    step: Vec3,
    state: Vec3
}

impl Transformation {
    pub fn empty() -> Transformation {
        Transformation {
            to: Vec3::zeros(),
            step: Vec3::zeros(),
            state: Vec3::zeros()
        }
    }

    pub fn new(to: Vec3) -> Transformation {
        Transformation {
            to,
            step: to / 5.0,
            state: Vec3::zeros()
        }
    }

    pub fn move_up(dy: f64) -> Transformation {
        Transformation::new(Vec3::new(0.0, -dy, 0.0))
    }

    pub fn move_down(dy: f64) -> Transformation {
        Transformation::new(Vec3::new(0.0, dy, 0.0))
    }

    pub fn rot_left(d: f64) -> Transformation {
        Transformation::new(Vec3::new(0.0, 0.0, -d))
    }

    pub fn rot_right(d: f64) -> Transformation {
        Transformation::new(Vec3::new(0.0, 0.0, d))            
    }

    pub fn x(&self) -> f64 {
        self.state.x
    }

    pub fn y(&self) -> f64 {
        self.state.y
    }

    pub fn rot(&self) -> f64 {
        self.state.z
    }

    pub fn update(&mut self, dt: f64) {
        // TODO use .approx_eq
        if self.step != Vec3::zeros() && (self.state - self.to).abs() <= Vec3::new(1.0, 1.0, 1.0) {
            self.step = Vec3::zeros();
            self.state = self.to;
        } else {
            self.state = self.state + self.step;
        }
    }
}

impl ::std::ops::Add for Transformation {
    type Output = Transformation;

    fn add(self, rhs: Transformation) -> Transformation {
        let new_to = self.to + rhs.to;
        Transformation {
            to: new_to,
            step: (new_to - self.state) / 10.0,
            state: self.state
        }
    }

}
