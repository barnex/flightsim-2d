use crate::prelude::*;

/// GPU buffer for uniform data,
/// corresponding to `ShaderPack::UNIFORM_LAYOUT`.
pub struct UniformBuffer<T> {
	buffer: wgpu::Buffer,
	binding: wgpu::BindGroup,
	_phantom: PhantomData<T>,
}

impl<T> UniformBuffer<T>
where
	T: bytemuck::Pod + Default,
{
	pub fn new(device: &wgpu::Device, data: T) -> Self {
		let buffer = Self::make_buffer(device, data);
		let binding = Self::make_bind_group(device, &buffer);

		Self {
			buffer,
			binding,
			_phantom: PhantomData,
		}
	}

	pub fn upload(&self, queue: &wgpu::Queue, data: &T) {
		queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(data));
	}

	/// Binds this buffer to @binding(0) (corresponding to `ShaderPack::UNIFORM_LAYOUT`).
	pub fn binding(&self) -> &wgpu::BindGroup {
		&self.binding
	}

	fn make_buffer(device: &wgpu::Device, data: T) -> wgpu::Buffer {
		device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some(file!()),
			contents: bytemuck::cast_slice(&[data]),
			usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
		})
	}

	fn make_bind_group(device: &wgpu::Device, buffer: &wgpu::Buffer) -> wgpu::BindGroup {
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some(file!()),
			layout: &device.create_bind_group_layout(&ShaderPack::UNIFORM_LAYOUT),
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
		})
	}
}
