pub fn to_bytes_big_endian(value: u32) -> [u8; 4] {
    [
        (value >> 24) as u8,
        (value >> 16) as u8,
        (value >> 8) as u8,
        value as u8,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_zero() {
        assert_eq!([0, 0, 0, 0], to_bytes_big_endian(0));
    }

    #[test]
    fn converts_number() {
        assert_eq!([0x1, 0x2, 0x3, 0x4], to_bytes_big_endian(16_909_060));
    }
}
