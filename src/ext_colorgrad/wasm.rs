//! Converts a [`Color`] object to primitives.

use colorgrad::Color;

pub fn color2rgba(c: Color) -> (u8, u8, u8, u8) {
    c.to_linear_rgba_u8()
}

pub fn color2u32(c: Color) -> u32 {
    let (r, g, b, a) = color2rgba(c);

    let r5: u32 = r.into();
    let g5: u32 = g.into();
    let b5: u32 = b.into();
    let a5: u32 = a.into();

    (r5 << 24) | (g5 << 16) | (b5 << 8) | a5
}
