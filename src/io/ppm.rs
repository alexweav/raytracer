use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

use crate::io::Image;
use crate::vector::Vector;

pub struct Ppm {
    file: File,
}

impl Ppm {
    #[allow(dead_code)]
    pub fn new(path: &Path, image_width: i32, image_height: i32) -> Self {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();

        Self::write_header(&mut file, image_width, image_height).unwrap();

        Self { file }
    }

    fn write_header(stream: &mut impl Write, width: i32, height: i32) -> Result<(), Error> {
        stream.write_all(b"P3\n")?;
        stream.write_all(format!("{} {}\n", width, height).as_bytes())?;
        stream.write_all(b"255\n")?;
        Ok(())
    }
}

impl Image for Ppm {
    fn write_pixel(&mut self, color: &Vector) {
        let row = format!(
            "{} {} {}\n",
            color.x() as i32,
            color.y() as i32,
            color.z() as i32
        );
        self.file.write_all(row.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn image_file_with_header_created() {
        let path = Path::new("test.ppm");
        Ppm::new(path, 10, 10);
        assert!(path.exists());

        let contents = fs::read_to_string(path).unwrap();
        assert_eq!("P3\n10 10\n255\n", contents);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn emits_formatted_ppm_pixels() {
        let path = Path::new("test.ppm");
        let mut ppm = Ppm::new(path, 10, 10);
        
        ppm.write_pixel(&Vector::new(1.0, 2.0, 3.0));
        ppm.write_pixel(&Vector:: new(128.5, 128.0, 255.5));

        let contents = fs::read_to_string(path).unwrap();
        assert_eq!("P3\n10 10\n255\n1 2 3\n128 128 255\n", contents);

        fs::remove_file(path).unwrap();
    }
}
