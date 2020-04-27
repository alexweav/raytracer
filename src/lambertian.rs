use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Lambertian {
    albedo: Vector,
}

impl Lambertian {
    pub fn new(albedo: &Vector) -> Lambertian {
        Lambertian {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vector, Ray) {
        let scatter_direction = &hit_record.normal + Vector::random_unit();
        //let scatter_direction = &hit_record.normal + Vector::random_in_hemisphere(&hit_record.normal);  // hemispherical scattering
        let scattered = Ray::new(&hit_record.p, &scatter_direction);
        let attenuation = self.albedo.clone();
        (true, attenuation, scattered)
    }
}
