use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Lambertian {
        Lambertian {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let scatter_direction = &hit_record.normal + Vec3::random_unit();
        //let scatter_direction = &hit_record.normal + Vec3::random_in_hemisphere(&hit_record.normal);  // hemispherical scattering
        let scattered = Ray::new(&hit_record.p, &scatter_direction);
        let attenuation = self.albedo.clone();
        (true, attenuation, scattered)
    }
}
