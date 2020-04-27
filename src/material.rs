use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vector, Ray);
}
