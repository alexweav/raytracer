use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vector, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo.clone(),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    fn reflect(v: &Vector, n: &Vector) -> Vector {
        v - (2.0 * Vector::dot(v, n) * n)
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vector, Ray) {
        let reflected = Self::reflect(&ray_in.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(
            &hit_record.p,
            &(reflected + self.fuzz * Vector::random_in_unit_sphere()),
        );
        let attenuation = self.albedo.clone();
        let scatter = Vector::dot(scattered.direction(), &hit_record.normal) > 0.0;
        (scatter, attenuation, scattered)
    }
}
