// Meshes for primitive shapes.

use crate::prelude::*;

pub fn unit_cube() -> MeshBuffer<TerrainVertex> {
	let mut mesh = MeshBuffer::new();

	let faces = [
		[((0, 1, 1), (1, 1)), ((0, 1, 0), (1, 0)), ((0, 0, 0), (0, 0)), ((0, 0, 1), (0, 1))], // left
		[((1, 0, 0), (0, 0)), ((1, 1, 0), (1, 0)), ((1, 1, 1), (1, 1)), ((1, 0, 1), (0, 1))], // right
		[((0, 0, 0), (0, 0)), ((1, 0, 0), (1, 0)), ((1, 0, 1), (1, 1)), ((0, 0, 1), (0, 1))], // bottom
		[((1, 1, 1), (1, 1)), ((1, 1, 0), (1, 0)), ((0, 1, 0), (0, 0)), ((0, 1, 1), (0, 1))], // top
		[((0, 1, 0), (0, 1)), ((1, 1, 0), (1, 1)), ((1, 0, 0), (1, 0)), ((0, 0, 0), (0, 0))], // back
		[((1, 0, 1), (1, 0)), ((1, 1, 1), (1, 1)), ((0, 1, 1), (0, 1)), ((0, 0, 1), (0, 0))], // front
	];

	for face in faces {
		mesh.push_rect(&face.map(|(pos, tex)| TerrainVertex::new(vec::from(pos).as_f32() - 0.5)))
	}

	mesh
}

pub fn cube_top() -> MeshBuffer<TerrainVertex> {
	let mut mesh = MeshBuffer::new();

	let face = [(0, 0), (1, 0), (1, 1), (0, 1)];

	mesh.push_rect(&face.map(|pos| TerrainVertex::new(vec::from(pos).append(1).as_f32() - 0.5)));

	mesh
}
