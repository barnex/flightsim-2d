use crate::prelude::*;

/// High-level wrapper around a WGPU Texture, View and Sampler,
/// corresponding to `ShaderPack::TEXTURE_LAYOUT`.
pub struct Texture {
	pub texture: wgpu::Texture,
	pub view: wgpu::TextureView,
	pub sampler: wgpu::Sampler,
	pub binding: wgpu::BindGroup,
}

pub fn load_embedded_atlas(device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Texture> {
	//let img = image::load_from_memory(include_bytes!("atlas.png"))?.flipv(); // 👈 flip because images are stored with Y down, but we use Y up.
	let img = image::load_from_memory(include_bytes!("../../../../assets/atlas.png"))?.flipv(); // 👈 flip because images are stored with Y down, but we use Y up.

	let mips = gen_mips(&img)?;
	let mips = mips.iter().map(|v| v.as_ref()).collect_vec();

	let opts = TextureOpts {
		format: wgpu::TextureFormat::Rgba8Unorm, // << assumes shaders work in SRGB space
		max_filter: wgpu::FilterMode::Linear,
		min_filter: wgpu::FilterMode::Linear,
		mipmap_filter: wgpu::FilterMode::Linear,
		address_mode: wgpu::AddressMode::ClampToEdge,
	};

	Ok(Texture::upload(device, queue, img.dimensions().into(), &mips, &opts))
}

impl Texture {
	pub fn uniform(device: &wgpu::Device, queue: &wgpu::Queue, rgba_color: vec4<u8>) -> Self {
		Self::upload(device, queue, vec2u(1, 1), &[&rgba_color.0], &TextureOpts::SRGB)
	}

	pub fn upload(device: &wgpu::Device, queue: &wgpu::Queue, dimensions: vec2u, mips: &[&[u8]], opts: &TextureOpts) -> Self {
		debug_assert!(mips[0].len() == 4 * dimensions.reduce(u32::mul) as usize);
		if mips.len() > 1 {
			assert!(dimensions.x().is_power_of_two());
			assert!(dimensions.y().is_power_of_two());
		}

		let mut size = wgpu::Extent3d {
			width: dimensions.x(),
			height: dimensions.y(),
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&wgpu::TextureDescriptor {
			label: Some(file!()),
			size,
			mip_level_count: mips.len() as u32,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: opts.format,
			view_formats: &[opts.format.add_srgb_suffix(), opts.format.remove_srgb_suffix()],
			usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
		});

		for (i, rgba) in mips.iter().enumerate() {
			let i = i as u32;
			queue.write_texture(
				wgpu::ImageCopyTexture {
					aspect: wgpu::TextureAspect::All,
					texture: &texture,
					mip_level: i,
					origin: wgpu::Origin3d::ZERO,
				},
				rgba,
				wgpu::ImageDataLayout {
					offset: 0,
					bytes_per_row: Some(4 * size.width),
					rows_per_image: Some(size.height),
				},
				size,
			);
			size.width /= 2;
			size.height /= 2;
		}

		let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = Self::default_sampler(device, opts);
		let binding = Self::make_bind_group(device, &view, &sampler);

		Texture { texture, view, sampler, binding }
	}

	/// Binds this texture + sampler to @binding(0), @binding(1) (corresponding to `ShaderPack::TEXTURE_LAYOUT`).
	pub fn binding(&self) -> &wgpu::BindGroup {
		&self.binding
	}

	pub fn make_bind_group(device: &wgpu::Device, view: &wgpu::TextureView, sampler: &wgpu::Sampler) -> wgpu::BindGroup {
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &device.create_bind_group_layout(&ShaderPack::TEXTURE_LAYOUT),
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0, // shader: @binding(0)
					resource: wgpu::BindingResource::TextureView(view),
				},
				wgpu::BindGroupEntry {
					binding: 1, // shader: @binding(1)
					resource: wgpu::BindingResource::Sampler(sampler),
				},
			],
			label: Some(file!()),
		})
	}

	pub fn default_sampler(device: &wgpu::Device, opts: &TextureOpts) -> wgpu::Sampler {
		device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("Texture::sampler"),
			address_mode_u: opts.address_mode,
			address_mode_v: opts.address_mode,
			address_mode_w: opts.address_mode,
			mag_filter: opts.max_filter,
			min_filter: opts.min_filter,
			mipmap_filter: opts.mipmap_filter,
			anisotropy_clamp: 1,
			..default()
		})
	}
}
