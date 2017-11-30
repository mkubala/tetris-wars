use na::Isometry2;

pub trait Movable {
    fn mov_up(&mut self);
    fn mov_down(&mut self);
    fn rot_left(&mut self);
    fn rot_right(&mut self);
    fn update(&mut self, dt: f64);

    fn apply(&mut self, iso: Isometry2<f64>);
}