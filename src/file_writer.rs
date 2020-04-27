use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::vec3::Vec3;

pub struct PpmWriter {
    file: File,
}

impl PpmWriter {
    pub fn new(path: &Path, image_width: i32, image_height: i32) -> PpmWriter {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();

        PpmWriter::write_header(&mut file, image_width, image_height);

        PpmWriter { file }
    }

    pub fn write_pixel(&mut self, vec: &Vec3, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (scale * vec.x()).sqrt();
        let g = (scale * vec.y()).sqrt();
        let b = (scale * vec.z()).sqrt();

        let row = format!(
            "{} {} {}\n",
            (256.0 * Self::clamp(r, 0.0, 0.999)) as i32,
            (256.0 * Self::clamp(g, 0.0, 0.999)) as i32,
            (256.0 * Self::clamp(b, 0.0, 0.999)) as i32
        );
        self.file.write_all(row.as_bytes()).unwrap();
    }

    fn write_header(stream: &mut impl Write, width: i32, height: i32) {
        stream.write_all(b"P3\n").unwrap();
        stream
            .write_all(format!("{} {}\n", width, height).as_bytes())
            .unwrap();
        stream.write_all(b"255\n").unwrap();
    }

    fn clamp(x: f64, min: f64, max: f64) -> f64 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }
}
