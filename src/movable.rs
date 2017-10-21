pub trait Movable {
    fn mov_up(&mut self, dy: f64);
    fn mov_down(&mut self, dy: f64);
    fn rot_left(&mut self);
    fn rot_right(&mut self);
    fn update(&mut self, dt: f64);
}