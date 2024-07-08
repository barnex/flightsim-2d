use crate::prelude::*;

/// Used by quads.wgsl.
/// Data passed to each instance for instanced rendering.
///
/// ! `repr(C)` required by WGPU.
/// ! Must be kept in sync with shaders.
/// ! Fields must be aligned to WGPU requirements
#[repr(C, align(16))]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable, EguiInspect)]
pub struct QuadInstanceData {
	pub mix_color: vec4f,       // 4
	pub position: vec3f,        // 7
	pub _padding: f32,          // 8
	pub tex_coords_off: vec2f,  // 10
	pub tex_coords_size: vec2f, // 12
	pub scale: vec2f,           // 13
	pub rotation: f32,          // 14
	pub _padding2: f32,         // 16
}

impl QuadInstanceData {
	pub fn new(position: vec2f, sprite: Sprite) -> Self {
		let (tex_coords_off, tex_coords_size) = index_atlas(sprite);
		Self {
			mix_color: vec4f::default(),
			position: position.append(0.0),
			scale: vec::splat(1.0),
			rotation: 0.0,
			tex_coords_off,
			tex_coords_size,
			_padding: default(),
			_padding2: default(),
		}
	}

	#[must_use = "does not modify original"]
	pub fn mix_color(mut self, color: vec4f) -> Self {
		self.mix_color = color;
		self
	}
}

fn index_atlas(sprite: Sprite) -> (vec2f, vec2f) {
	const LOG_SPRITES_PER_ROW: u16 = 3;
	const SPRITES_PER_ROW: u16 = 1 << LOG_SPRITES_PER_ROW;
	const STRIDE: f32 = SPRITES_PER_ROW as f32;
	const MARGIN: f32 = 1.0 / 512.0; // 👈 TODO: tune to atlas size so it's 1 pixel.

	let (ix, iy) = sprite.pos.into();
	let (ix, iy) = (ix as f32, iy as f32);
	let size = sprite.size.as_f32();

	let tex_coords_off = vec2f(ix / STRIDE, 1.0 - (iy + 1.0) / STRIDE) + MARGIN;
	let tex_coords_size = size / STRIDE - (2.0 * MARGIN);
	(tex_coords_off, tex_coords_size)
}
