extern crate adler32;

use crate::io::util;

pub fn compress(data: &[u8]) -> Vec<u8> {
    const CHUNK_SIZE: usize = 65530;

    let num_chunks = {
        let filled_chunks = data.len() / CHUNK_SIZE;
        filled_chunks
            + if data.len() == filled_chunks * CHUNK_SIZE && data.len() != 0 {
                0
            } else {
                1
            }
    };
    let final_size = 2 + (num_chunks * 5) + data.len() + 4;

    let mut compressed = Vec::with_capacity(final_size);
    compressed.extend(&[0x78, 0x01]);

    let mut checksum = adler32::RollingAdler32::new();

    for (idx, chunk) in data.chunks(CHUNK_SIZE).enumerate() {
        let last_byte = if idx == num_chunks - 1 {
            0x1 as u8
        } else {
            0x0 as u8
        };
        compressed.extend(&[last_byte]);
        let block_len: [u8; 4] = [
            (chunk.len() & 0xff) as u8,
            ((chunk.len() >> 8) & 0xff) as u8,
            (0xff - (chunk.len() & 0xff)) as u8,
            (0xff - ((chunk.len() >> 8) & 0xff)) as u8,
        ];
        compressed.extend(&block_len);
        compressed.extend(chunk);
        checksum.update_buffer(&chunk);
    }

    compressed.extend(&util::to_bytes_big_endian(checksum.hash()));

    compressed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_compress_empty_buffer_no_blocks() {
        let expected = vec![
            0x78, 0x01, // header
            0x0, 0x0, 0x0, 0x1, // checksum
        ];
        assert_eq!(expected, compress(&[]));
    }

    #[test]
    fn zero_compress_simple_buffer() {
        let expected = vec![
            0x78, 0x01, // header
            0x1,  // last chunk
            0x3, 0x0, // len
            0xfc, 0xff, // nlen
            0x1, 0x2, 0x3, // data
            0x0, 0xd, 0x0, 0x7, // checksum
        ];
        assert_eq!(expected, compress(&[1, 2, 3]));
    }
}
