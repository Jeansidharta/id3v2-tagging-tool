pub fn check_bit(byte: u8, bit_index: u8) -> bool {
    (byte >> bit_index) & 1 == 1
}

/// A syncsafe integer is a 32 byte integer where every byte has the MSB set to zero.
pub fn read_syncsafe_intefer(bytes: &[u8; 4]) -> u32 {
    (bytes[3] as u32) | (bytes[2] as u32) << 7 | (bytes[1] as u32) << 14 | (bytes[0] as u32) << 21
}

pub fn write_syncsafe_integer(number: u32) -> [u8; 4] {
    [
        (((number << 3) & 0b01111111_00000000_00000000_00000000) >> 24) as u8,
        (((number << 2) & 0b00000000_01111111_00000000_00000000) >> 16) as u8,
        (((number << 1) & 0b00000000_00000000_01111111_00000000) >> 8) as u8,
        (((number << 0) & 0b00000000_00000000_00000000_01111111) >> 0) as u8,
    ]
}
