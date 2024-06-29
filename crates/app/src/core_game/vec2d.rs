use crate::prelude::*;

/// 2D Array.
#[derive(Clone, Serialize, Deserialize)]
pub struct Vec2D<T> {
	pub size: vec2u,
	pub values: Vec<T>,
}

impl<T> EguiInspect for Vec2D<T>
//where
//T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			ui.label(&format!("{} x {}", self.size.x(), self.size.y()))
		});
	}

	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.inspect(label, ui)
	}
}

impl<T> Vec2D<T>
where
	T: Clone,
{
	pub fn new(size: vec2u, fill: T) -> Self {
		Self {
			size,
			values: (0..(size.x() * size.y())).map(|_| fill.clone()).collect(),
		}
	}
}

impl<T> Vec2D<T>
where
	T: Clone + Default,
{
	pub fn clear(&mut self) {
		let default = T::default();
		self.values.iter_mut().for_each(|v| *v = default.clone())
	}

	pub fn at_or_default(&self, pos: Pos) -> T {
		self.try_at(pos).cloned().unwrap_or_default()
	}

	pub fn iter_range_excl(&self, bounds: Bounds2i) -> impl Iterator<Item = (Pos, T)> + '_ {
		// TODO: too many checks and conversions
		bounds
			.intersect(self.bounds().as_i32())
			.as_u32()
			.iter_excl()
			.map(|pos| (pos.as_i32(), self.at_or_default(pos.as_i32())))
	}
}

impl<T> Vec2D<T> {
	pub fn try_at(&self, pos: Pos) -> Option<&T> {
		self.bounds().as_i32().contains(pos).then(|| self._at(pos.as_u32()))
	}

	pub fn _at(&self, pos: vec2u) -> &T {
		#[cfg(debug_assertions)]
		if !(pos.x() < self.size.x() && pos.y() < self.size.y()) {
			panic!("Vec2D::at({pos}): out of range {:?}", self.bounds())
		}
		&self.values[(pos.x() + self.size.x() * pos.y()) as usize]
	}

	pub fn at_mut(&mut self, pos: vec2u) -> &mut T {
		debug_assert!(pos.x() < self.size.x() && pos.y() < self.size.y());
		&mut self.values[(pos.x() + self.size.x() * pos.y()) as usize]
	}

	pub fn try_at_mut(&mut self, pos: Pos) -> Option<&mut T> {
		self.bounds().as_i32().contains(pos).then(|| self.at_mut(pos.as_u32()))
	}

	pub fn try_set(&mut self, pos: Pos, value: T) {
		if self.bounds().as_i32().contains(pos) {
			*self.at_mut(pos.as_u32()) = value;
		}
	}

	pub fn iter(&self) -> impl Iterator<Item = (vec2u, &T)> + '_ {
		//self.iter_positions().enumerate().map(|(i, pos)| (pos, &self.values[i]))
		self.iter_positions().map(|pos| (pos, self._at(pos)))
	}

	pub fn iter_positions(&self) -> impl Iterator<Item = vec2u> {
		(0..self.size.y()).cartesian_product(0..self.size.x()).map(|(x, y)| vec2(y, x)) // 👈 must transpose so that x is inner loop
	}
}

impl<T> Vec2D<Option<T>> {
	pub fn take_at(&mut self, pos: Pos) -> Option<T> {
		match self.try_at_mut(pos) {
			None => None,
			Some(ptr) => Option::take(ptr),
		}
	}
}

impl<T> Vec2D<T> {
	pub fn bounds(&self) -> Bounds2u {
		Bounds { min: vec::ZERO, max: self.size }
	}
}

impl<T> fmt::Debug for Vec2D<T>
where
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Vec2D").field("size", &self.size).field("values", &self.values.len()).finish()
	}
}
