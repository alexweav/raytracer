mod camera;
mod color;
mod dielectric;
mod environments;
mod hittable;
mod io;
mod lambertian;
mod material;
mod metal;
mod ray;
mod render;
mod scenery;
mod sphere;
mod vector;

extern crate clap;
#[macro_use]
extern crate impl_ops;
extern crate rand;

use clap::{App, Arg};
use std::path::Path;

use crate::io::Png;
use crate::render::render_scenery;

fn main() {
    let matches = App::new("raytracer")
        .version("0.1.0")
        .author("Alex Weaver <weaver.alex.d@gmail.com>")
        .about("Renders 3D images using brute-force pathtracing")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("The path of the rendered image.")
                .takes_value(true),
        )
        .get_matches();

    let file_location = matches.value_of("output").unwrap_or("pngtest.png");
    let environment = environments::spheres::get_environment();
    /*let mut file = PpmWriter::new(
        Path::new(file_location),
        environment.image.width,
        environment.image.height,
    );*/
    let mut file = Png::new(
        Path::new(file_location),
        environment.image.width,
        environment.image.height,
    );
    render_scenery(
        environment.scenery,
        &environment.camera,
        (environment.image.width, environment.image.height),
        &mut file,
        environment.image.supersampling_ratio,
    );
    println!("\nDone.");
}
