mod vec3;
use vec3::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) {
        Ray { origin, dir }
    }

    pub fn at(t: f64) -> Point3 {
        origin + t * dir
    }
}
