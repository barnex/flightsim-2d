use crate::prelude::*;

pub struct Renderer {
	device: Arc<wgpu::Device>,
	queue: Arc<wgpu::Queue>,
	shader_pack: ShaderPack,
	framebuf: Framebuffer,

	uniform_buffer: UniformBuffer<UniformData>,
	instance_buffer: StorageBuffer<QuadInstanceData>,

	vertex_buffer: StorageBuffer<TerrainVertex>,
	index_buffer: wgpu::Buffer,
	num_indices: u32,

	texture_atlas: Texture,

	// draw instances from instance buffer layer-by-layer, back to front.
	layer_boundaries: Vec<u32>,
}

pub const RED: vec4<u8> = vec4(255, 0, 0, 255);
pub const BLUE: vec4<u8> = vec4(0, 0, 255, 255);

/// GPU buffers byte size is multiple of this.
/// wgpu does not like arbitrarily sized buffers.
pub const BUFFER_QUANTUM_BYTES: u64 = 64 * 1024;

impl Renderer {
	// Fixed instance buffer byte size. Instances that don't fit won't be rendered.
	const INSTANCE_BUFFER_BYTES: u64 = 4 * 1024 * 1024;

	pub fn new(device: &Arc<wgpu::Device>, queue: &Arc<wgpu::Queue>, target_format: wgpu::TextureFormat, opts: &GraphicsOpts) -> Self {
		Self {
			device: device.clone(),
			queue: queue.clone(),
			shader_pack: ShaderPack::new(device, target_format, opts),
			framebuf: Framebuffer::new(device, vec2(32, 32) /* initial size to change on first frame*/),
			uniform_buffer: UniformBuffer::new(device, UniformData::default()),
			instance_buffer: StorageBuffer::<QuadInstanceData>::new(device, Self::INSTANCE_BUFFER_BYTES, &ShaderPack::INSTANCE_LAYOUT),
			vertex_buffer: StorageBuffer::<TerrainVertex>::new(device, BUFFER_QUANTUM_BYTES, &ShaderPack::VERTEX_LAYOUT),
			index_buffer: new_index_buffer(device, BUFFER_QUANTUM_BYTES),
			num_indices: 0,
			texture_atlas: load_embedded_atlas(device, queue).log_err().unwrap_or_else(|_| Texture::uniform(device, queue, RED)),
			layer_boundaries: default(),
		}
	}

	pub fn prepare(&mut self, encoder: &mut wgpu::CommandEncoder, sg: &Scenegraph, viewport_size: vec2u) -> Vec<wgpu::CommandBuffer> {
		self.framebuf.rightsize(&self.device, viewport_size);

		self.uniform_buffer.upload(&self.queue, &sg.uniforms);

		self.instance_buffer.upload_with_resize(&self.device, &self.queue, sg.instances());

		self.vertex_buffer.upload_with_resize(&self.device, &self.queue, sg.meshbuffer.vertices());
		upload_index_buffer(&self.device, &self.queue, sg.meshbuffer.indices(), &mut self.index_buffer);
		self.num_indices = sg.meshbuffer.indices.len() as u32;

		self.layer_boundaries.clear();
		self.layer_boundaries.extend_from_slice(&sg.layer_boundaries);
		self.layer_boundaries.push(sg.instances.len() as u32);

		// This render pass renders the world to the offscreen HDR framebuffer.
		{
			let (r, g, b, a) = sg.clear_color.as_f64().into();
			let render_pass_desc = wgpu::RenderPassDescriptor {
				label: Some("offscreen render pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &self.framebuf.hdr_texture.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color { r, g, b, a }),
						store: wgpu::StoreOp::Store,
					},
				})],
				depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
					view: &self.framebuf.depth_texture_view,
					depth_ops: Some(wgpu::Operations {
						load: wgpu::LoadOp::Clear(0.0),
						store: wgpu::StoreOp::Store,
					}),
					stencil_ops: None,
				}),

				occlusion_query_set: None,
				timestamp_writes: None,
			};
			let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

			render_pass.set_bind_group(ShaderPack::UNIFORM_GROUP, self.uniform_buffer.binding(), &[]); // bind  @group(0)
			render_pass.set_bind_group(ShaderPack::TEXTURE_GROUP, self.texture_atlas.binding(), &[]); // bind @group(2)

			// 3D
			{
				render_pass.set_pipeline(&self.shader_pack.terrain_pipeline);
				render_pass.set_bind_group(ShaderPack::VERTEX_GROUP, self.vertex_buffer.binding(), &[]); // bind @group(1)
				render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

				let num_indices = {
					// clamp indices to draw so they fit in the index buffer
					// in rare cases where we attempt rendering too much, this will
					// cause visual artifacts (missing triangles) instead of crashing wgpu.
					let index_buffer_len = (self.index_buffer.size() / mem::size_of::<u32>() as u64) as u32;
					if self.num_indices > index_buffer_len {
						log::error!("index buffer len ({}) < number of indices ({})", index_buffer_len, self.num_indices);
					}
					self.num_indices.clamp(0, index_buffer_len)
				};
				render_pass.draw_indexed(0..num_indices, 0, 0..1);
			}
			// 2D
			{
				render_pass.set_pipeline(&self.shader_pack.quads_pipeline);
				render_pass.set_bind_group(ShaderPack::INSTANCE_GROUP, self.instance_buffer.binding(), &[]); // bind @group(1)

				for (start, end) in iter::once(0).chain(self.layer_boundaries.iter().copied()).tuple_windows() {
					// ☠️ TODO: don't draw if end > buffer len!
					render_pass.draw(0..6 /* 2 triangles */, start..end);
				}
			}
		}

		vec![]
	}

	pub fn paint<'rpass>(&'rpass self, render_pass: &mut wgpu::RenderPass<'rpass>) {
		// This (trivial) render pass transfers the HDR framebuffer to the screen.

		// LOL egui uses these
		render_pass.set_bind_group(ShaderPack::UNIFORM_GROUP, self.uniform_buffer.binding(), &[]); // bind  @group(0)
		render_pass.set_bind_group(ShaderPack::INSTANCE_GROUP, self.instance_buffer.binding(), &[]); // bind @group(1)

		render_pass.set_bind_group(ShaderPack::TEXTURE_GROUP, &self.framebuf.hdr_texture.binding, &[]); // bind @group(2)
		render_pass.set_pipeline(&self.shader_pack.copy_framebuffer);
		render_pass.draw(0..6 /* 2 triangles */, 0..1);
	}
}
