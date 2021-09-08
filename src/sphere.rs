use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}
impl HitRecord {
    pub fn from(
        p: Point3,
        mut normal: Vec3,
        mat_ptr: Box<dyn Material>,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
        normal = if front_face { normal } else { -normal };
        HitRecord {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }
    pub fn get_outward_normal(r: &Ray, outward_normal: Vec3) -> bool {
        Vec3::dot(r.direction, outward_normal) < 0.0
    }
}
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Box<dyn Material>,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64, mat_ptr: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat_ptr,
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}
impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        for hittable in self {
            if let Some(thing) = hittable.hit(ray, tmin, tmax) {
                return Some(thing);
            }
        }

        None
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
        let mut root = (-half_b - sqrtd) / a;
        if root < tmin || tmax < root {
            root = (-half_b + sqrtd) / a;

            if root < tmin || tmax < root {
                return None;
            }
        }

        let p = ray.at(root);
        let rec = HitRecord::from(
            p,
            (p - self.center) / self.radius,
            self.mat_ptr,
            root,
            HitRecord::get_outward_normal(ray, (p - self.center) / self.radius),
        );

        Some(rec)
    }
}
