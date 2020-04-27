use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

pub struct Scenery {
    objects: Vec<Box<dyn Hittable>>,
}

impl Scenery {
    pub fn empty() -> Scenery {
        Scenery {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
    }
}

impl Hittable for Scenery {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut last_hit_record = HitRecord::empty();
        let mut closest_so_far = t_max;

        for object in &self.objects {
            let (hit, record) = object.hit(ray, t_min, closest_so_far);
            if hit {
                hit_anything = true;
                closest_so_far = record.t;
                last_hit_record = record;
            }
        }

        (hit_anything, last_hit_record)
    }
}
