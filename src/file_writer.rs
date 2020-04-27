use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::vector::Vector;

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

    pub fn write_pixel(&mut self, color: &Vector) {
        let row = format!(
            "{} {} {}\n",
            color.x() as i32,
            color.y() as i32,
            color.z() as i32
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
}
