use crate::prelude::*;

// Shared mutability (get/set/push/remove).
// Elements cannot be referenced, only cloned in/out.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CellVec<T> {
	inner: RefCell<Vec<T>>,
}

impl<T> CellVec<T>
where
	T: Clone,
{
	pub fn len(&self) -> usize {
		self.inner.borrow().len()
	}

	pub fn get(&self, index: usize) -> Option<T> {
		self.inner.borrow().get(index).cloned()
	}

	pub fn first(&self) -> Option<T> {
		self.inner.borrow().first().cloned()
	}

	pub fn last(&self) -> Option<T> {
		self.inner.borrow().last().cloned()
	}

	pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
		let n = self.inner.borrow().len();
		(0..n).filter_map(|i| self.get(i))
	}

	pub fn retain<F: Fn(T) -> bool>(&self, f: F) {
		self.inner.borrow_mut().retain(|v| f(v.clone()))
	}

	pub fn set_from_iter<I: IntoIterator<Item = T>>(&self, values: I) {
		*self.inner.borrow_mut() = values.into_iter().collect();
	}

	pub fn set(&self, inner: Vec<T>) {
		*self.inner.borrow_mut() = inner;
	}

	pub fn clear(&self) {
		self.inner.borrow_mut().clear();
	}

	pub fn get_mut(&mut self) -> &mut Vec<T> {
		self.inner.get_mut()
	}

	pub fn push_now(&mut self, value: T) {
		self.get_mut().push(value)
	}
}

impl<T> CellVec<T> {
	pub const fn new() -> Self {
		Self { inner: RefCell::new(Vec::new()) }
	}
}

impl<T> From<Vec<T>> for CellVec<T> {
	fn from(inner: Vec<T>) -> Self {
		Self { inner: RefCell::new(inner) }
	}
}

impl<T> Default for CellVec<T> {
	fn default() -> Self {
		Self { inner: default() }
	}
}

impl<A> FromIterator<A> for CellVec<A> {
	fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
		Self {
			inner: RefCell::new(Vec::from_iter(iter)),
		}
	}
}

impl<T> EguiInspect for CellVec<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.inner.borrow().inspect(label, ui)
	}
}
