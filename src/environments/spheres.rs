use rand::Rng;
use std::rc::Rc;

use crate::camera::{Camera, CameraConfig};
use crate::dielectric::Dielectric;
use crate::environments::environment::{Environment, Image};
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::scenery::Scenery;
use crate::sphere::Sphere;
use crate::vector::Vector;

pub fn get_environment() -> Environment {
    let image = make_image();
    Environment {
        camera: make_camera(&image),
        image,
        scenery: make_world(),
    }
}

fn make_image() -> Image {
    Image {
        width: 1200,
        height: 600,
        supersampling_ratio: 50,
    }
}

fn make_camera(image: &Image) -> Camera {
    let config = CameraConfig {
        location: Vector::new(13.0, 2.0, 3.0),
        lookat: Vector::new(0.0, 0.0, 0.0),
        up: Vector::new(0.0, 1.0, 0.0),
        field_of_view: 20.0,
        aspect_ratio: image.width as f64 / image.height as f64,
        focus_distance: 10.0,
        aperture: 0.1,
    };
    Camera::from_config(config)
}

fn make_world() -> Scenery {
    let mut world = Scenery::empty();
    world.add(Box::new(Sphere::new(
        &Vector::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(&Vector::new(0.5, 0.5, 0.5))),
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Vector::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (&center - Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    let albedo = Vector::random(&mut rng) * Vector::random(&mut rng);
                    world.add(Box::new(Sphere::new(
                        &center,
                        0.2,
                        Rc::new(Lambertian::new(&albedo)),
                    )));
                } else if choose_mat < 0.90 {
                    let albedo = Vector::random_range(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    world.add(Box::new(Sphere::new(
                        &center,
                        0.2,
                        Rc::new(Metal::new(&albedo, fuzz)),
                    )));
                } else {
                    world.add(Box::new(Sphere::new(
                        &center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )));
                };
            }
        }
    }

    world.add(Box::new(Sphere::new(
        &Vector::new(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        &Vector::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::new(&Vector::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        &Vector::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::new(&Vector::new(0.7, 0.6, 0.5), 0.0)),
    )));
    world
}
