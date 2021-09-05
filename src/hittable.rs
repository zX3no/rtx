use crate::ray::Ray;
use crate::sphere::HitRecord;
use crate::vec3::{Color, Point3, Vec3};

pub struct HittableList {
    objects: Vec<Self>,
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = &mut temp_rec;
            }
        }

        return hit_anything;
    }
}
