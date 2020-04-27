use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

use crate::camera::Camera;
use crate::color::to_color;
use crate::file_writer::PpmWriter;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::scenery::Scenery;
use crate::vector::Vector;

pub fn render_scenery(
    scene: Scenery,
    camera: &Camera,
    image_size: (i32, i32),
    file: &mut PpmWriter,
    supersampling_ratio: i32,
) {
    let max_depth = 50;
    let (image_width, image_height) = image_size;
    let mut rng = rand::thread_rng();
    println!("Rendering scene...");
    let progress_bar = ProgressBar::new(image_height as u64 * image_width as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {pos}/{len}px, eta {eta} "),
    );
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Vector::new(0.0, 0.0, 0.0);
            for _ in 0..supersampling_ratio {
                let u = (i as f64 + rng.gen::<f64>()) / image_width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / image_height as f64;
                let ray = camera.get_ray(u, v);
                color = color + ray_color(ray, &scene, &mut rng, max_depth);
            }
            //color.gamma_correct(supersampling_ratio);
            let color = to_color(color, supersampling_ratio);
            file.write_pixel(&color);
            progress_bar.inc(1);
        }
    }
    progress_bar.finish();
}

fn ray_color(ray: Ray, world: &impl Hittable, rng: &mut impl rand::Rng, depth: i32) -> Vector {
    if depth <= 0 {
        return Vector::new(0.0, 0.0, 0.0);
    }

    let (hit, record) = world.hit(&ray, 0.001, std::f64::INFINITY);
    if hit {
        let material = record.material.as_ref().unwrap();
        let (scatter, attenuation, scattered) = material.scatter(&ray, &record);
        if scatter {
            attenuation * ray_color(scattered, world, rng, depth - 1)
        } else {
            Vector::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + (t * Vector::new(0.5, 0.7, 1.0))
    }
}
