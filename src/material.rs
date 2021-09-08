use crate::ray::Ray;
use crate::sphere::HitRecord;
use crate::vec3::Color;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: Color, scattered: &Ray) -> bool;
}
