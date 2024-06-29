use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ID<T> {
	pub index: u32, // TODO: private
	pub generation: u32,
	_marker: PhantomData<T>,
}

impl<T> EguiInspect for ID<T> {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			ui.label(format!("{self:?}"))
		});
	}
}

impl<T> ID<T> {
	// TODO: make private, move id.rs to cell_arena/id.rs
	pub fn new(index: usize, generation: u32) -> Self {
		Self {
			index: index as u32,
			generation,
			_marker: PhantomData,
		}
	}

	pub fn index(&self) -> usize {
		self.index as usize
	}

	pub fn untyped(&self) -> (usize, u32) {
		(self.index as usize, self.generation)
	}
}

impl<T> PartialEq for ID<T> {
	fn eq(&self, other: &Self) -> bool {
		self.untyped() == other.untyped()
	}
}

impl<T> PartialOrd for ID<T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl<T> Ord for ID<T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.untyped().cmp(&other.untyped())
	}
}

impl<T> Eq for ID<T> {}

impl<T> std::hash::Hash for ID<T> {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.untyped().hash(state)
	}
}

impl<T> fmt::Debug for ID<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ID({}.{})", self.index, self.generation)
	}
}

impl<T> Clone for ID<T> {
	fn clone(&self) -> Self {
		*self
	}
}
impl<T> Copy for ID<T> {}

impl<T> Default for ID<T> {
	fn default() -> Self {
		Self {
			index: 0,
			generation: 0,
			_marker: PhantomData,
		}
	}
}
