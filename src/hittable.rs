use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;
use std::rc::Rc;

pub struct HitRecord {
    pub p: Vector,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord);
}

impl HitRecord {
    pub fn empty() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vector::new(0.0, 0.0, 0.0),
            normal: Vector::new(0.0, 0.0, 0.0),
            front_face: false,
            material: None,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector) {
        self.front_face = Vector::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        }
    }
}
