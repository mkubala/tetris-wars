use na::Isometry2;

pub trait Movable {
    fn update(&mut self, dt: f64);

    fn apply(&mut self, iso: Isometry2<f64>);
}