use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::Path;

extern crate crc32fast;
use crc32fast::Hasher;

use crate::io::Image;
use crate::vector::Vector;

#[allow(dead_code)]
pub struct PngWriter {
    file: File,
    data: Vec<u8>,
}

#[allow(dead_code)]
enum ChunkType {
    Header,
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
            data: Vec::new()
        }
    }

    fn write_header(stream: &mut impl Write, width: i32, height: i32) -> Result<(), Error> {
        let signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        stream.write_all(&signature)?;
        let w = Self::to_bytes_big_endian(width as u32);
        let h = Self::to_bytes_big_endian(height as u32);
        let header = [
            w[0], w[1], w[2], w[3],
            h[0], h[1], h[2], h[3],
            8, 6, 0, 0, 0
        ];
        Self::write_chunk(stream, ChunkType::Header, &header)?;
        Ok(())
    }

    fn write_chunk(stream: &mut impl Write, chunk_type: ChunkType, data: &[u8]) -> Result<(), Error> {
        let chunk_length = data.len() as u32;
        stream.write(&Self::to_bytes_big_endian(chunk_length))?;
        let mut crc32 = Hasher::new();
        let chunk_type = match chunk_type {
            ChunkType::Header => b"IHDR",
            ChunkType::End => b"IEND",
        };
        crc32.update(chunk_type);
        stream.write(chunk_type)?;
        crc32.update(data);
        stream.write(data)?;
        let checksum = crc32.finalize();
        stream.write(&Self::to_bytes_big_endian(checksum))?;
        Ok(())
    }

    fn to_bytes_big_endian(value: u32) -> [u8; 4] {
        [(value >> 24) as u8, (value >> 16) as u8, (value >> 8) as u8, value as u8]
    }
}

impl Image for PngWriter {
    fn write_pixel(&mut self, _: &Vector) {

    }
}

impl Drop for PngWriter {
    fn drop(&mut self) {
        Self::write_chunk(&mut self.file, ChunkType::End, &[]).unwrap();
    }
}
