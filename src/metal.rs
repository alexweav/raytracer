use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo.clone(),
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - (2.0 * Vec3::dot(v, n) * n)
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let reflected = Self::reflect(&ray_in.direction().unit_vector(), &hit_record.normal);
        let scattered = Ray::new(
            &hit_record.p,
            &(reflected + self.fuzz * Vec3::random_in_unit_sphere()),
        );
        let attenuation = self.albedo.clone();
        let scatter = Vec3::dot(scattered.direction(), &hit_record.normal) > 0.0;
        (scatter, attenuation, scattered)
    }
}
