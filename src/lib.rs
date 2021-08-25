pub fn blend_u8(a: u8, b: u8, mask: u8) -> u8{ a ^ ((a ^ b) & mask) }

pub fn blend_u16(a: u16, b: u16, mask: u16) -> u16{ a ^ ((a ^ b) & mask) }

pub fn blend_u32(a: u32, b: u32, mask: u32) -> u32{ a ^ ((a ^ b) & mask) }

pub fn blend_u64(a: u64, b: u64, mask: u64) -> u64{ a ^ ((a ^ b) & mask) }

pub fn blend_u128(a: u128, b: u128, mask: u128) -> u128{ a ^ ((a ^ b) & mask) }

pub fn bool_to_u8_mask(a: bool) -> u8{
    match a{
        true => { u8::MAX }
        false => { 0 }
    }
}

pub fn bool_to_u16_mask(a: bool) -> u16{
    match a{
        true => { u16::MAX }
        false => { 0 }
    }
}

pub fn bool_to_u32_mask(a: bool) -> u32{
    match a{
        true => { u32::MAX }
        false => { 0 }
    }
}

pub fn bool_to_u64_mask(a: bool) -> u64{
    match a{
        true => { u64::MAX }
        false => { 0 }
    }
}

pub fn bool_to_u128_mask(a: bool) -> u128{
    match a{
        true => { u128::MAX }
        false => { 0 }
    }
}

#[inline(always)]
pub fn u8_mask_to_bool(a: u8) -> bool{ a > 0 }

#[inline(always)]
pub fn u16_mask_to_bool(a: u16) -> bool{ a > 0 }

#[inline(always)]
pub fn u32_mask_to_bool(a: u32) -> bool{ a > 0 }

#[inline(always)]
pub fn u64_mask_to_bool(a: u64) -> bool{ a > 0 }

#[inline(always)]
pub fn u128_mask_to_bool(a: u128) -> bool{ a > 0 }
