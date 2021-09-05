use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
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
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction, *outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}
impl Sphere {
    pub fn from(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
impl Hittable for Vec<Box<Hittable>> {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for hitable in self {
            if let Some(candidate_hit) = hitable.hit(ray, tmin, tmax) {
                match hit {
                    None => hit = Some(candidate_hit),
                    Some(prev) => {
                        if candidate_hit.t < prev.t {
                            hit = Some(candidate_hit);
                        }
                    }
                }
            }
        }

        hit
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range.
        let mut root = -half_b - sqrtd / a;
        if root < tmin || tmax < root {
            root = -half_b + sqrtd / a;

            if root < tmin || tmax < root {
                return None;
            }
        }
        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.normal = rec.p - self.center / self.radius;

        let outward_normal = rec.p - self.center / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        return Some(rec);
    }
}
