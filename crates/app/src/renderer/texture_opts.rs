use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct TextureOpts {
	pub format: wgpu::TextureFormat,
	pub min_filter: wgpu::FilterMode,
	pub max_filter: wgpu::FilterMode,
	pub mipmap_filter: wgpu::FilterMode,
	pub address_mode: wgpu::AddressMode,
}

impl TextureOpts {
	pub const DEFAULT: Self = Self::RGBA_AS_IS;

	pub const SRGB: Self = Self {
		format: wgpu::TextureFormat::Rgba8UnormSrgb,
		max_filter: wgpu::FilterMode::Linear,
		min_filter: wgpu::FilterMode::Linear,
		mipmap_filter: wgpu::FilterMode::Nearest,
		address_mode: wgpu::AddressMode::Repeat,
	};

	// RGB in whatever format was in the original.
	pub const RGBA_AS_IS: TextureOpts = TextureOpts {
		format: wgpu::TextureFormat::Rgba8Unorm,
		..Self::SRGB
	};
}

impl Default for TextureOpts {
	fn default() -> Self {
		Self::DEFAULT
	}
}
