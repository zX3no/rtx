use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}
pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::from(0.0, 0.0, 0.0),
            normal: Vec3::from(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: true,
        }
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let mut root = -half_b - sqrtd / a;
        if root < t_min || t_max < root {
            root = -half_b + sqrtd / a;

            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.normal = rec.p - self.center / self.radius;

        let outward_normal = rec.p - self.center / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        return true;
    }
}
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction, *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}
