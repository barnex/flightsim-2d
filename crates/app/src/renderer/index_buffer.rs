use crate::prelude::*;

pub fn new_index_buffer(device: &wgpu::Device, len: u64) -> wgpu::Buffer {
	let size = mem::size_of::<u32>() as u64 * len;
	device.create_buffer(&wgpu::BufferDescriptor {
		label: Some(file!()),
		size,
		mapped_at_creation: false,
		usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
	})
}

/// Upload index buffer `src` bytes to `dest` on the GPU.
/// Auto-resize `dest` if too small.
pub fn upload_index_buffer(device: &wgpu::Device, queue: &wgpu::Queue, src: &[u32], dest: &mut wgpu::Buffer) {
	let src_bytes = bytemuck::cast_slice(src);

	if src_bytes.len() as u64 > dest.size() {
		// TODO: have a maximum size
		let new_size = round_to(src_bytes.len() as u64, BUFFER_QUANTUM_BYTES);
		log::trace!("index buffer of size {} is full, resizing to {new_size}", dest.size());
		*dest = new_index_buffer(device, new_size);
	}

	queue.write_buffer(dest, 0 /*offset*/, src_bytes);
}
