use crate::prelude::*;

#[inline]
pub fn pack4xu8(v: impl Into<vec4<u8>>) -> u32 {
	let v = v.into();
	(v[0] as u32) | (v[1] as u32) << 8 | (v[2] as u32) << 16 | (v[3] as u32) << 24
}
