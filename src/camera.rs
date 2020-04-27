use crate::ray::Ray;
use crate::vector::Vector;

pub struct CameraConfig {
    pub location: Vector,
    pub lookat: Vector,
    pub up: Vector,
    pub field_of_view: f64,
    pub aspect_ratio: f64,
    pub focus_distance: f64,
    pub aperture: f64,
}

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    //w: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: &Vector,
        lookat: &Vector,
        vup: &Vector,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let origin = lookfrom.clone();
        let lens_radius = aperture / 2.0;
        let theta = Self::degrees_to_radians(vfov);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = Vector::cross(&vup, &w).unit_vector();
        let v = Vector::cross(&w, &u);
        Camera {
            lower_left_corner: &origin
                - (half_width * focus_dist * &u)
                - (half_height * focus_dist * &v)
                - (focus_dist * &w),
            origin: origin,
            horizontal: 2.0 * half_width * focus_dist * &u,
            vertical: 2.0 * half_height * focus_dist * &v,
            u: u,
            v: v,
            //w: w,
            lens_radius: lens_radius,
        }
    }

    pub fn from_config(config: CameraConfig) -> Camera {
        Self::new(
            &config.location,
            &config.lookat,
            &config.up,
            config.field_of_view,
            config.aspect_ratio,
            config.aperture,
            config.focus_distance,
        )
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vector::random_in_unit_disk();
        let offset = (&self.u * rd.x()) + (&self.v * rd.y());
        Ray::new(
            &(&self.origin + &offset),
            &(&self.lower_left_corner + (s * &self.horizontal) + (t * &self.vertical)
                - &self.origin
                - offset),
        )
    }

    fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }
}
