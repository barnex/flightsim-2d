use crate::prelude::*;
use std::borrow::Borrow;

/// RcCell with lazy init
#[derive(Serialize, Debug, Deserialize, Default, Clone)]
pub struct LazyCell<T>(RefCell<Option<Rc<T>>>);

impl<T> LazyCell<T>
where
	T: Clone,
{
	pub fn new(value: T) -> Self {
		Self(RefCell::new(Some(Rc::new(value))))
	}

	pub fn get_or_init(&self, f: impl FnOnce() -> T) -> Rc<T> {
		let mut inner = self.0.borrow_mut();
		match &*inner {
			Some(v) => v.clone(),
			None => {
				let v = Rc::new(f());
				*inner = Some(v.clone());
				v
			}
		}
	}

	pub fn unset(&self) {
		*self.0.borrow_mut() = None
	}

	/// Force set value, regarless of being initialized.
	pub fn set(&self, value: T) {
		*self.0.borrow_mut() = Some(Rc::new(value))
	}

	pub fn try_get(&self) -> Option<Rc<T>> {
		self.0.borrow().clone()
	}
}

impl<T> EguiInspect for LazyCell<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		let inner = self.0.borrow();
		match &*inner {
			Some(v) => v.inspect(label, ui),
			None => {
				ui.label(label);
			}
		}
	}
}
