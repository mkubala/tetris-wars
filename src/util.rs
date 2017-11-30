pub mod isometry {
    use ::na::Isometry2;

    pub fn add_isometries(lhs: &Isometry2<f64>, rhs: &Isometry2<f64>) -> Isometry2<f64> {
        Isometry2::new(
            lhs.translation.vector.clone() + rhs.translation.vector.clone(), 
            lhs.rotation.angle() + rhs.rotation.angle())
    }

    pub fn sub_isometries(lhs: &Isometry2<f64>, rhs: &Isometry2<f64>) -> Isometry2<f64> {
        Isometry2::new(
            lhs.translation.vector.clone() - rhs.translation.vector.clone(), 
            lhs.rotation.angle() - rhs.rotation.angle())
    }

    pub fn div_isometry(iso: &Isometry2<f64>, div: f64) -> Isometry2<f64> {
        Isometry2::new(
            iso.translation.vector.clone() / div, 
            iso.rotation.angle() / div)
    }
}