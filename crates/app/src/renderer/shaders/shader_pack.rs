use crate::prelude::*;

/// Collection of mutually compatible shaders (same bind group layouts etc).
pub(crate) struct ShaderPack {
	pub quads_pipeline: wgpu::RenderPipeline,   // quads.wgsl, but FB_FORMAT for off-screen
	pub terrain_pipeline: wgpu::RenderPipeline, // terrain.wgsl, but FB_FORMAT for off-screen
	pub copy_framebuffer: wgpu::RenderPipeline, // copy_framebuffer.wgsl
}

impl ShaderPack {
	/// All shaders in the `ShaderPack` bind their uniforms under group 0.
	pub const UNIFORM_GROUP: u32 = 0;

	/// All shaders in the `ShaderPack` bind their instance data array under group 1.
	pub const INSTANCE_GROUP: u32 = 1;

	/// All shaders in the `ShaderPack` bind their vertex data array under group 1.
	pub const VERTEX_GROUP: u32 = 1;

	/// All shaders in the `ShaderPack` bind their textures+samples under group 2.
	pub const TEXTURE_GROUP: u32 = 2;

	pub fn new(device: &wgpu::Device, target_format: wgpu::TextureFormat, opts: &GraphicsOpts) -> Self {
		Self {
			quads_pipeline: Self::make_quads_pipeline(device, "quads", include_str!("quads.wgsl")),
			terrain_pipeline: Self::make_terrain_pipeline(device, "terrain", include_str!("terrain.wgsl")),
			copy_framebuffer: Self::make_egui_pipeline(device, "copy_framebuffer", include_str!("copy_framebuffer.wgsl"), target_format),
		}
	}

	fn make_terrain_pipeline(device: &wgpu::Device, label: &'static str, source: &str) -> wgpu::RenderPipeline {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some(label),
			source: wgpu::ShaderSource::Wgsl(source.into()),
		});

		let bind_group_layouts = [
			device.create_bind_group_layout(&ShaderPack::UNIFORM_LAYOUT), // shader: @group(0)
			device.create_bind_group_layout(&ShaderPack::VERTEX_LAYOUT),  // shader: @group(1)
			device.create_bind_group_layout(&ShaderPack::TEXTURE_LAYOUT), // shader: @group(2)
		];

		device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some(label),

			layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some(label),
				bind_group_layouts: &bind_group_layouts.each_ref(),
				push_constant_ranges: &[],
			})),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: Framebuffer::HDR_FORMAT,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::OVER,
						alpha: wgpu::BlendComponent::OVER,
					}),
					write_mask: default(),
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Cw,
				cull_mode: Some(wgpu::Face::Front),
				//cull_mode: None,
				unclipped_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			//depth_stencil: None,
			depth_stencil: Some(wgpu::DepthStencilState {
				format: Framebuffer::DEPTH_FORMAT,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::GreaterEqual,
				stencil: wgpu::StencilState::default(),
				bias: wgpu::DepthBiasState::default(),
			}),
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		})
	}

	// Shader that renders to the offscreen HDR framebuffer (with depth stencil etc).
	fn make_quads_pipeline(device: &wgpu::Device, label: &'static str, source: &str) -> wgpu::RenderPipeline {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some(label),
			source: wgpu::ShaderSource::Wgsl(source.into()),
		});

		let bind_group_layouts = [
			device.create_bind_group_layout(&ShaderPack::UNIFORM_LAYOUT),  // shader: @group(0)
			device.create_bind_group_layout(&ShaderPack::INSTANCE_LAYOUT), // shader: @group(1)
			device.create_bind_group_layout(&ShaderPack::TEXTURE_LAYOUT),  // shader: @group(2)
		];

		device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some(label),

			layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some(label),
				bind_group_layouts: &bind_group_layouts.each_ref(),
				push_constant_ranges: &[],
			})),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: Framebuffer::HDR_FORMAT,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::OVER,
						alpha: wgpu::BlendComponent::OVER,
					}),
					write_mask: default(),
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				unclipped_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			//depth_stencil: None,
			depth_stencil: Some(wgpu::DepthStencilState {
				format: Framebuffer::DEPTH_FORMAT,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::GreaterEqual,
				stencil: wgpu::StencilState::default(),
				bias: wgpu::DepthBiasState::default(),
			}),
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		})
	}

	// Shader that renders to egui's surface. Target format chosen by egui, no depth buffer etc.
	fn make_egui_pipeline(device: &wgpu::Device, label: &'static str, source: &str, target_format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
		let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
			label: Some(label),
			source: wgpu::ShaderSource::Wgsl(source.into()),
		});

		let bind_group_layouts = [
			device.create_bind_group_layout(&ShaderPack::UNIFORM_LAYOUT),  // shader: @group(0)
			device.create_bind_group_layout(&ShaderPack::INSTANCE_LAYOUT), // shader: @group(1)
			device.create_bind_group_layout(&ShaderPack::TEXTURE_LAYOUT),  // shader: @group(2)
		];

		device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some(label),

			layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some(label),
				bind_group_layouts: &bind_group_layouts.each_ref(),
				push_constant_ranges: &[],
			})),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: target_format.remove_srgb_suffix(),
					blend: None,
					write_mask: default(),
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: None,
				unclipped_depth: false,
				polygon_mode: wgpu::PolygonMode::Fill,
				conservative: false,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState::default(),
			multiview: None,
		})
	}

	/// Layout of the uniform buffer, identical for all shaders in the `ShaderPack`.
	pub const UNIFORM_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some("ShaderPack::UNIFORM_LAYOUT"),
		entries: &[wgpu::BindGroupLayoutEntry {
			binding: 0,
			visibility: wgpu::ShaderStages::VERTEX,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Uniform,
				has_dynamic_offset: false,
				min_binding_size: None,
			},
			count: None,
		}],
	};

	pub const INSTANCE_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some(file!()),
		entries: &[wgpu::BindGroupLayoutEntry {
			binding: 0,
			visibility: wgpu::ShaderStages::VERTEX,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Storage { read_only: true },
				has_dynamic_offset: false,
				min_binding_size: None,
			},
			count: None,
		}],
	};

	pub const VERTEX_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some(file!()),
		entries: &[wgpu::BindGroupLayoutEntry {
			binding: 0,
			visibility: wgpu::ShaderStages::VERTEX,
			ty: wgpu::BindingType::Buffer {
				ty: wgpu::BufferBindingType::Storage { read_only: true },
				has_dynamic_offset: false,
				min_binding_size: None,
			},
			count: None,
		}],
	};

	pub const TEXTURE_LAYOUT: wgpu::BindGroupLayoutDescriptor<'static> = wgpu::BindGroupLayoutDescriptor {
		label: Some(file!()),
		entries: &[
			// Binding 0: texture data
			wgpu::BindGroupLayoutEntry {
				binding: 0, // shader: @binding(0)
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Texture {
					multisampled: false,
					view_dimension: wgpu::TextureViewDimension::D2,
					sample_type: wgpu::TextureSampleType::Float { filterable: true },
				},
				count: None,
			},
			// Binding 1: texture sampler
			wgpu::BindGroupLayoutEntry {
				binding: 1, // shader: @binding(1)
				visibility: wgpu::ShaderStages::FRAGMENT,
				ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
				count: None,
			},
		],
	};
}
