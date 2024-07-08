use crate::prelude::*;

/// GPU counterpart of `Vec<T>`: a growable buffer of elements,
/// corresponding to `ShaderPack::INSTANCE_LAYOUT`.
pub struct StorageBuffer<T> {
	buffer: wgpu::Buffer,
	binding: wgpu::BindGroup, // binds buffer to `@binding(0)`, where `ShaderPack` expects it.
	layout: wgpu::BindGroupLayoutDescriptor<'static>,
	_phantom: PhantomData<T>,
}

impl<T> StorageBuffer<T>
where
	T: Pod + Default,
{
	pub fn new(device: &wgpu::Device, bytes: u64, layout: &wgpu::BindGroupLayoutDescriptor<'static>) -> Self {
		assert!(
			mem::size_of::<T>() % 16 == 0,
			"StorageBuffer<{}>: invalid alignment: {}, must be multiple of 16",
			std::any::type_name::<T>(),
			mem::size_of::<T>()
		);
		let buffer = Self::new_buffer(device, bytes);
		let binding = Self::make_bind_group(device, &buffer, layout);
		Self {
			buffer,
			binding,
			layout: layout.to_owned(),
			_phantom: PhantomData,
		}
	}

	/// Like `upload`, but grows the buffer if needed to fit `data`.
	pub fn upload_with_resize(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T]) {
		let bytes = bytemuck::cast_slice(data);
		if bytes.len() as u64 > self.buffer.size() {
			let new_size = round_to(bytes.len() as u64, BUFFER_QUANTUM_BYTES);
			log::trace!("resizing storage buffer from {} B to {new_size} B", self.buffer.size());
			self.resize(device, new_size);
		}
		self.upload_bytes(device, queue, bytes)
	}

	/// Uploads `data` to the GPU, overwriting previous content.
	/// Sets the buffer's `len`.
	pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, data: &[T]) {
		self.upload_bytes(device, queue, bytemuck::cast_slice(data))
	}

	fn upload_bytes(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, bytes: &[u8]) {
		let mut n = bytes.len();
		if bytes.len() as u64 > self.buffer.size() {
			log::error!("storage buffer of size {} is full", self.buffer.size());
			n = self.buffer.size() as usize;
		}
		queue.write_buffer(&self.buffer, 0 /*offset*/, &bytes[..n]);
	}

	fn resize(&mut self, device: &wgpu::Device, bytes: u64) {
		self.buffer = Self::new_buffer(device, bytes);
		self.binding = Self::make_bind_group(device, &self.buffer, &self.layout);
	}

	/// Binds this buffer to @binding(0) (corresponding to `ShaderPack::INSTANCE_LAYOUT`).
	pub fn binding(&self) -> &wgpu::BindGroup {
		&self.binding
	}

	fn new_buffer(device: &wgpu::Device, bytes: u64) -> wgpu::Buffer {
		device.create_buffer(&wgpu::BufferDescriptor {
			label: Some(file!()),
			size: bytes,
			mapped_at_creation: false,
			usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
		})
	}

	fn make_bind_group(device: &wgpu::Device, buffer: &wgpu::Buffer, layout: &wgpu::BindGroupLayoutDescriptor) -> wgpu::BindGroup {
		let bind_group_layout = device.create_bind_group_layout(layout);
		device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some(file!()),
			layout: &bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: buffer.as_entire_binding(),
			}],
		})
	}
}

pub fn round_to(n: u64, to: u64) -> u64 {
	(((n - 1) / to) + 1) * to
}
