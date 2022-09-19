pub mod latin1;
pub mod log;

pub fn check_bit(byte: u8, bit_index: u8) -> bool {
    (byte >> bit_index) & 1 == 1
}

/// A syncsafe integer is a 32 byte integer where every byte has the MSB set to zero.
pub fn read_syncsafe_integer(bytes: &[u8; 4]) -> u32 {
    (bytes[3] as u32) | (bytes[2] as u32) << 7 | (bytes[1] as u32) << 14 | (bytes[0] as u32) << 21
}

pub fn read_be_integer(bytes: &[u8; 4]) -> u32 {
    (bytes[3] as u32) | (bytes[2] as u32) << 8 | (bytes[1] as u32) << 16 | (bytes[0] as u32) << 24
}

pub fn write_syncsafe_integer(number: u32) -> [u8; 4] {
    [
        (((number << 3) & 0b01111111_00000000_00000000_00000000) >> 24) as u8,
        (((number << 2) & 0b00000000_01111111_00000000_00000000) >> 16) as u8,
        (((number << 1) & 0b00000000_00000000_01111111_00000000) >> 8) as u8,
        (((number << 0) & 0b00000000_00000000_00000000_01111111) >> 0) as u8,
    ]
}

pub fn write_be_integer(number: u32) -> [u8; 4] {
    [
        (((number << 3) & 0b11111111_00000000_00000000_00000000) >> 24) as u8,
        (((number << 2) & 0b00000000_11111111_00000000_00000000) >> 16) as u8,
        (((number << 1) & 0b00000000_00000000_11111111_00000000) >> 8) as u8,
        (((number << 0) & 0b00000000_00000000_00000000_11111111) >> 0) as u8,
    ]
}

pub fn is_valid_syncsafe_integer(bytes: &[u8; 4]) -> bool {
    // Check if any byte has the last bit set
    (bytes[0] | bytes[1] | bytes[2] | bytes[3]) >> 7 == 0
}
