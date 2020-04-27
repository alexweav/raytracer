use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;
use std::rc::Rc;

pub struct Sphere {
    center: Vector,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: &Vector, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: center.clone(),
            radius,
            material,
        }
    }

    pub fn center(&self) -> &Vector {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let center_offset = ray.origin() - self.center();
        let a = ray.direction().length_squared();
        let half_b = Vector::dot(&center_offset, ray.direction());
        let c = center_offset.length_squared() - (self.radius() * self.radius());
        let discriminant = (half_b * half_b) - (a * c);

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let outward_normal = (&p - self.center()) / self.radius();
                /*let mut hit_record = HitRecord::new(
                    &p,
                    &outward_normal,
                    temp,
                    self.material.clone(),
                );*/
                let mut hit_record = HitRecord {
                    t: temp,
                    p: p,
                    normal: Vector::new(0.0, 0.0, 0.0),
                    front_face: false,
                    material: Some(self.material.clone()),
                };
                hit_record.set_face_normal(ray, &outward_normal);
                return (true, hit_record);
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.at(temp);
                let outward_normal = (&p - self.center()) / self.radius();
                /*let mut hit_record = HitRecord::new(
                    &p,
                    &outward_normal,
                    temp,
                    self.material.clone(),
                );*/
                let mut hit_record = HitRecord {
                    t: temp,
                    p: p,
                    normal: Vector::new(0.0, 0.0, 0.0),
                    front_face: false,
                    material: Some(self.material.clone()),
                };
                hit_record.set_face_normal(ray, &outward_normal);
                return (true, hit_record);
            }
        }

        (false, HitRecord::empty())
    }
}
