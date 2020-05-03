use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

extern crate crc32fast;
use crc32fast::Hasher;

use crate::io::deflate_zero;
use crate::io::util;
use crate::io::Image;
use crate::vector::Vector;

#[allow(dead_code)]
pub struct PngWriter {
    file: File,
    data: Vec<u8>,
    width: i32,
    height: i32,
}

#[allow(dead_code)]
enum ChunkType {
    Header,
    Data,
    End,
}

#[allow(dead_code)]
impl PngWriter {
    pub fn new(path: &Path, width: i32, height: i32) -> PngWriter {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path)
            .unwrap();
        PngWriter::write_header(&mut file, width, height).unwrap();
        PngWriter {
            file,
            data: Vec::with_capacity(((4 * width + 1) * height) as usize),
            width: width,
            height: height,
        }
    }

    fn write_header(stream: &mut impl Write, width: i32, height: i32) -> Result<(), Error> {
        let signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        stream.write_all(&signature)?;
        let w = util::to_bytes_big_endian(width as u32);
        let h = util::to_bytes_big_endian(height as u32);
        let header = [
            w[0], w[1], w[2], w[3], h[0], h[1], h[2], h[3], 8, 6, 0, 0, 0,
        ];
        Self::write_chunk(stream, ChunkType::Header, &header)?;
        Ok(())
    }

    fn write_chunk(
        stream: &mut impl Write,
        chunk_type: ChunkType,
        data: &[u8],
    ) -> Result<(), Error> {
        let chunk_length = data.len() as u32;
        stream.write_all(&util::to_bytes_big_endian(chunk_length))?;
        let mut crc32 = Hasher::new();
        let chunk_type = match chunk_type {
            ChunkType::Header => b"IHDR",
            ChunkType::Data => b"IDAT",
            ChunkType::End => b"IEND",
        };
        crc32.update(chunk_type);
        stream.write_all(chunk_type)?;
        crc32.update(data);
        stream.write_all(data)?;
        let checksum = crc32.finalize();
        stream.write_all(&util::to_bytes_big_endian(checksum))?;
        Ok(())
    }
}

impl Image for PngWriter {
    fn write_pixel(&mut self, color: &Vector) {
        // Prepend the 0 format token to each scanline.
        if self.data.len() % (self.width * 4 + 1) as usize == 0 {
            self.data.push(0);
        }
        let row = [color.x() as u8, color.y() as u8, color.z() as u8, 0xff];
        self.data.extend_from_slice(&row);
    }
}

impl Drop for PngWriter {
    fn drop(&mut self) {
        let compressed = deflate_zero::compress(&self.data[..]);
        Self::write_chunk(&mut self.file, ChunkType::Data, &compressed).unwrap();
        Self::write_chunk(&mut self.file, ChunkType::End, &[]).unwrap();
    }
}
