use crate::prelude::*;

/// Uniform data passed to shaders.
///
/// ! `repr(C)` required by WGPU.
/// ! Must be kept in sync with shaders.
/// ! Fields must be aligned to WGPU requirements
#[repr(C, align(16))]
#[derive(Clone, Copy, Pod, Zeroable, Default, Debug, EguiInspect)]
pub struct UniformData {
	pub camera: mat4x4f,
}
