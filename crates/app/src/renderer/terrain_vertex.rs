use crate::prelude::*;

/// Vertex data used by shader `terrain.wgsl`.
///
/// ! `repr(C)` required by WGPU.
/// ! Must be kept in sync with terrain.wgsl.
/// ! Fields must be aligned to WGPU requirements
/// ! Total size must be align(16)
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable, EguiInspect)]
pub struct TerrainVertex {
	pub position: vec3f,
	pub color: u32,
}

impl TerrainVertex {
	pub fn new(position: vec3f) -> Self {
		Self {
			position,
			color: pack4xu8(vec::splat(255)),
		}
	}
}
