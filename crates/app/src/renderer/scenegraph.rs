use crate::prelude::*;

#[derive(Clone, Debug, EguiInspect)]
pub struct Scenegraph {
	pub clear_color: vec4f,
	pub uniforms: UniformData,

	pub instances: Vec<QuadInstanceData>,

	pub meshbuffer: MeshBuffer<TerrainVertex>,

	// draw instances back-to-front, starting a new layer at each of these indices.
	pub curr_z: f32,

	#[inspect(hide)]
	pub layer_boundaries: Vec<u32>,
}

impl Scenegraph {
	const MAX_LAYERS: usize = 100;
	pub const TYP_LAYERS: usize = 8;

	pub fn clear(&mut self) {
		self.instances.clear();
		self.curr_z = 0.0;
		self.layer_boundaries.clear();
		self.meshbuffer.clear();
	}

	// Start a new layer, drawing over the instances added to the previous layer.
	pub fn new_layer(&mut self) {
		// Each layer issues a draw call, which is quite expensive.
		// Catch accidentally calling new_layer too often.
		debug_assert!(self.layer_boundaries.len() < Self::TYP_LAYERS);
		debug_assert!(self.layer_boundaries.len() < Self::MAX_LAYERS);

		if !self.instances.is_empty() {
			self.layer_boundaries.push(self.instances.len() as u32)
		}
		self.curr_z += 1.0 / 16.0; // 👈 next layer is a bit higher (arbitrary amount).
	}

	pub fn push(&mut self, instance: QuadInstanceData) {
		const Z_OFFSET_2D: f32 = 3.0; // 👈 offset 2D quads upwards by this amount so they are above the terrain.
		self.instances.push(instance.with(|i| i.position[2] = self.curr_z + Z_OFFSET_2D))
	}

	pub fn instances(&self) -> &[QuadInstanceData] {
		&self.instances
	}
}

impl Default for Scenegraph {
	fn default() -> Self {
		Self {
			clear_color: vec4f(0.0, 0.0, 1.0, 1.0),
			uniforms: UniformData::default(),
			instances: default(),
			curr_z: 0.0,
			layer_boundaries: default(),
			meshbuffer: default(),
		}
	}
}
