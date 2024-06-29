use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, EguiInspect)]
pub(crate) struct MeshBuffer<T> {
	pub vertices: Vec<T>,
	pub indices: Vec<u32>,
}

impl<T> MeshBuffer<T> {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn vertices(&self) -> &[T] {
		&self.vertices
	}

	pub fn indices(&self) -> &[u32] {
		&self.indices
	}

	/// Add a single vertex, assign it to the next free index.
	/// Vertices are typically pushed per 3.
	pub fn push(&mut self, v: T) {
		let index = self.vertices.len() as u32;
		self.vertices.push(v);
		self.indices.push(index);
	}

	pub fn clear(&mut self) {
		self.vertices.clear();
		self.indices.clear();
	}
}

impl<T> MeshBuffer<T>
where
	T: Clone,
{
	pub fn from<'i, 'v, V, I>(vertices: V, indices: I) -> Self
	where
		V: IntoIterator<Item = T> + 'v,
		I: IntoIterator<Item = &'i u32>,
		T: 'v,
	{
		Self::new().with(|v| v.extend(vertices, indices))
	}

	pub fn append(&mut self, rhs: &Self) {
		self.extend(rhs.vertices.iter().cloned(), &rhs.indices)
	}

	pub fn push_rect(&mut self, vertices: &[T; 4]) {
		self.extend(vertices.iter().cloned(), &[0, 1, 2, 0, 2, 3]);
	}

	pub fn push_triangle(&mut self, vertices: &[T; 3]) {
		self.extend(vertices.iter().cloned(), &[0, 1, 2]);
	}

	pub fn extend<'i, 'v, V, I>(&mut self, vertices: V, indices: I)
	where
		V: IntoIterator<Item = T> + 'v,
		I: IntoIterator<Item = &'i u32>,
		T: 'v,
	{
		let offset = self.vertices.len() as u32;
		self.indices.extend(indices.into_iter().map(|v| v + offset));
		self.vertices.extend(vertices.into_iter().map(|v| v.to_owned()));
	}

	// pub fn bounds(&self) -> Option<BoundingBox<f32>> {
	// 	BoundingBox::from_points(self.vertices().iter().map(|v| v.position))
	// }

	// pub fn collect<'a>(shards: impl IntoIterator<Item = &'a MeshBuffer>) -> Self {
	// 	let mut buf = Self::new();
	// 	for shard in shards {
	// 		buf.append(shard)
	// 	}
	// 	buf
	// }

	// pub fn line(start: vec3, end: vec3) -> Self {
	// 	let start = VertexLM { position: start, ..default() };
	// 	let end = VertexLM { position: end, ..default() };

	// 	Self {
	// 		vertices: vec![start, end],
	// 		indices: vec![0, 1],
	// 	}
	// }

	// #[must_use = "Does not modify the original"]
	// pub fn translated(&self, delta: vec3) -> Self {
	// 	self.map_positions(|p| p + delta)
	// }

	// pub fn transform(&mut self, transf: &mat4) {
	// 	for v in &mut self.vertices {
	// 		v.transform(transf)
	// 	}
	// }

	// A copy of `self`, with a function applied to the vertex positions.
	// TODO: transform normals, etc.
	//#[must_use = "Does not modify the original"]
	//pub fn map_positions<F>(&self, f: F) -> Self
	//where
	//	F: Fn(vec3) -> vec3,
	//{
	//	Self {
	//		indices: self.indices.clone(),
	//		vertices: self.vertices.iter().map(|v| v.map_position(&f)).collect(),
	//	}
	//}
}

impl<T> Default for MeshBuffer<T> {
	fn default() -> Self {
		Self {
			vertices: default(),
			indices: default(),
		}
	}
}
