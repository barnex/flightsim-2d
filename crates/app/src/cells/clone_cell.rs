use crate::prelude::*;
use std::borrow::Borrow;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct CloneCell<T>(RefCell<T>);

impl<T> CloneCell<T>
where
	T: Clone,
{
	pub fn new(value: T) -> Self {
		Self(RefCell::new(value))
	}

	pub fn set(&self, value: T) {
		*self.0.borrow_mut() = value
	}

	pub fn get_cloned(&self) -> T {
		self.0.borrow().clone()
	}
}

impl<T> CloneCell<Rc<[T]>>
where
	T: Clone,
{
	pub fn len(&self) -> usize {
		self.0.borrow().len()
	}

	pub fn get(&self, index: usize) -> Option<T> {
		self.0.borrow().get(index).cloned()
	}

	pub fn clear(&self) {
		*self.0.borrow_mut() = Rc::from([])
	}
}

impl<T> CloneCell<Option<T>> {
	pub fn is_none(&self) -> bool {
		self.0.borrow().is_none()
	}
	pub fn is_some(&self) -> bool {
		self.0.borrow().is_some()
	}
}

impl<T> From<T> for CloneCell<T>
where
	T: Clone,
{
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

impl<T> Clone for CloneCell<T>
where
	T: Clone,
{
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> EguiInspect for CloneCell<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.0.borrow().inspect(label, ui)
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.0.borrow_mut().inspect_mut(label, ui)
	}
}
