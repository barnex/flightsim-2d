use crate::prelude::*;

pub struct Framebuffer {
	pub hdr_texture: Texture,
	pub depth_texture: wgpu::Texture,
	pub depth_texture_view: wgpu::TextureView,
}

impl Framebuffer {
	pub const HDR_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float; // HDR-ready
	pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

	pub fn new(device: &wgpu::Device, size: vec2u) -> Self {
		let (depth_texture, depth_texture_view) = Self::depth_texture(device, size);
		Self {
			hdr_texture: Self::hdr_texture(device, size),
			depth_texture,
			depth_texture_view,
		}
	}

	pub fn rightsize(&mut self, device: &wgpu::Device, viewport_size: vec2u) {
		let (depth_texture, depth_texture_view) = Self::depth_texture(device, viewport_size);
		if self.size() != viewport_size {
			self.hdr_texture = Self::hdr_texture(device, viewport_size);
			self.depth_texture = depth_texture;
			self.depth_texture_view = depth_texture_view;
		}
	}

	fn size(&self) -> vec2u {
		vec2u(self.hdr_texture.texture.size().width, self.hdr_texture.texture.size().height)
	}

	fn hdr_texture(device: &wgpu::Device, size: vec2u) -> Texture {
		let fb = device.create_texture(&wgpu::TextureDescriptor {
			size: wgpu::Extent3d {
				width: size.x(),
				height: size.y(),
				depth_or_array_layers: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: Self::HDR_FORMAT,
			usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
			label: Some("offscreen buffer"),
			view_formats: &[], // <<<<<<<<<< ????
		});
		let view = fb.create_view(&default()); // <<<<<<<< default????
		let sampler = Texture::default_sampler(
			device,
			&TextureOpts {
				format: wgpu::TextureFormat::Rgba8UnormSrgb, // <<< // SRGB ??????????
				max_filter: wgpu::FilterMode::Nearest,
				min_filter: wgpu::FilterMode::Nearest,
				mipmap_filter: wgpu::FilterMode::Nearest,
				address_mode: wgpu::AddressMode::ClampToEdge,
			},
		);
		let bidning = Texture::make_bind_group(device, &view, &sampler);

		Texture {
			texture: fb,
			view,
			sampler,
			binding: bidning,
		}
	}

	fn depth_texture(device: &wgpu::Device, size: vec2u) -> (wgpu::Texture, wgpu::TextureView) {
		let size = wgpu::Extent3d {
			width: size.x(),
			height: size.y(),
			depth_or_array_layers: 1,
		};
		let desc = wgpu::TextureDescriptor {
			label: Some(file!()),
			size,
			mip_level_count: 1,
			//sample_count: opts.msaa_sample_count(),
			sample_count: 1, // <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
			dimension: wgpu::TextureDimension::D2,
			format: Self::DEPTH_FORMAT,
			view_formats: &[Self::DEPTH_FORMAT],
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
		};
		let texture = device.create_texture(&desc);
		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Linear,
			min_filter: wgpu::FilterMode::Linear,
			mipmap_filter: wgpu::FilterMode::Nearest,
			compare: Some(wgpu::CompareFunction::LessEqual),
			lod_min_clamp: 0.0,
			lod_max_clamp: 100.0,
			..default()
		});

		(texture, view)
	}
}
