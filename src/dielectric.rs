use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Self::ffmin(Vec3::dot(&-uv, n), 1.0);
        let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
        let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;
        r_out_parallel + r_out_perp
    }

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - (2.0 * Vec3::dot(v, n) * n)
    }

    fn ffmin(a: f64, b: f64) -> f64 {
        if a <= b {
            a
        } else {
            b
        }
    }

    fn schlick(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> (bool, Vec3, Ray) {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = Self::ffmin(Vec3::dot(&-&unit_direction, &hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = Self::schlick(cos_theta, etai_over_etat);
        if etai_over_etat * sin_theta > 1.0 || rand::thread_rng().gen::<f64>() < reflect_prob {
            let reflected = Self::reflect(&unit_direction, &hit_record.normal);
            let scattered = Ray::new(&hit_record.p, &reflected);
            (true, attenuation, scattered)
        } else {
            let refracted = Self::refract(&unit_direction, &hit_record.normal, etai_over_etat);
            let scattered = Ray::new(&hit_record.p, &refracted);
            (true, attenuation, scattered)
        }
    }
}
