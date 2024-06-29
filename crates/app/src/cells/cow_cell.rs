use crate::prelude::*;
use std::borrow::Borrow;

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct CowCell<T>(RefCell<Rc<T>>);

impl<T> CowCell<T> {
	pub fn new(value: T) -> Self {
		Self(RefCell::new(Rc::new(value)))
	}

	pub fn set(&self, value: T) {
		*self.0.borrow_mut() = Rc::new(value)
	}

	pub fn get(&self) -> Rc<T> {
		Rc::clone(&self.0.borrow())
	}
}

impl<T> CowCell<Option<T>> {
	pub fn is_none(&self) -> bool {
		self.0.borrow().is_none()
	}
	pub fn is_some(&self) -> bool {
		self.0.borrow().is_some()
	}
}

impl<T> From<T> for CowCell<T> {
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

impl<T> Clone for CowCell<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T> EguiInspect for CowCell<T>
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
