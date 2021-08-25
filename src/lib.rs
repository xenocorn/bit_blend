pub fn blend_u8(a: u8, b: u8, mask: u8) -> u8{ a ^ ((a ^ b) & mask) }

pub fn blend_u16(a: u16, b: u16, mask: u16) -> u16{ a ^ ((a ^ b) & mask) }

pub fn blend_u32(a: u32, b: u32, mask: u32) -> u32{ a ^ ((a ^ b) & mask) }

pub fn blend_u64(a: u64, b: u64, mask: u64) -> u64{ a ^ ((a ^ b) & mask) }

pub fn blend_u128(a: u128, b: u128, mask: u128) -> u128{ a ^ ((a ^ b) & mask) }
