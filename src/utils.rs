use core::convert::TryInto;


pub fn read_magic(bytes: &[u8]) -> u32 {
    let magic = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
    magic
}

pub fn read_u32_from_offset(bytes: &[u8], offset: usize) -> u32 {
    let value = u32::from_le_bytes(bytes[offset..(offset+4)].try_into().unwrap());
    value
}

pub fn read_i32_from_offset(bytes: &[u8], offset: usize) -> i32 {
    let value = i32::from_le_bytes(bytes[offset..(offset+4)].try_into().unwrap());
    value
}
