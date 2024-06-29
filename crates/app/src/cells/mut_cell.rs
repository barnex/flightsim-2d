use std::{
	fmt::Display,
	ops::{Deref, DerefMut},
};

use crate::prelude::*;

/// Cell on steroids. Known to #[derive(Setters)].
/// Makes a GameObject field mutable during the scripting stage.
pub struct Mut<T> {
	inner: Cell<T>,
}

impl<T: Default> Default for Mut<T> {
	fn default() -> Self {
		Self { inner: Cell::new(T::default()) }
	}
}

// BAD for compound types:
//   position.set((0, 0))
//   position.ptr()[X] = 1;
//   position.ptr()[Y] = 2;
//   // result will not be (1, 2), but (1, 0) or (0, 2)
//   // depending on drop order
//pub struct CGuard<'c, T: Copy> {
//	orig: &'c C<T>,
//	temp: T,
//}
//impl<'c, T: Copy> Deref for CGuard<'c, T> {
//	type Target = T;
//
//	fn deref(&self) -> &Self::Target {
//		&self.temp
//	}
//}
//

impl<T> Mut<T>
where
	T: Copy,
{
	const fn new(v: T) -> Self {
		Self { inner: Cell::new(v) }
	}

	pub fn get(&self) -> T {
		self.inner.get()
	}

	pub fn set(&self, v: T) {
		self.inner.set(v)
	}

	pub fn mutate<F: Fn(&mut T)>(&self, f: F) {
		let mut v = self.inner.get();
		f(&mut v);
		self.inner.set(v);
	}

	pub fn replace<F: Fn(T) -> T>(&self, f: F) {
		self.inner.set(f(self.inner.get()));
	}
}

impl<T> Clone for Mut<T>
where
	T: Copy,
{
	fn clone(&self) -> Self {
		Self { inner: self.inner.clone() }
	}
}

impl<T> Mut<T>
where
	T: Copy + Add<Output = T>,
{
	pub fn add(&self, v: T) {
		self.set(self.get() + v)
	}
}

impl<T> Add<T> for &Mut<T>
where
	T: Copy + Add<Output = T>,
{
	type Output = T;

	fn add(self, rhs: T) -> Self::Output {
		self.get() + rhs
	}
}

impl<T> Mut<T>
where
	T: Copy + Add<Output = T>,
{
	pub fn increment(&self, rhs: T) {
		self.set(self.get() + rhs)
	}
}

impl<T> Mut<T>
where
	T: Copy + Sub<Output = T>,
{
	pub fn decrement(&self, rhs: T) {
		self.set(self.get() - rhs)
	}
}

impl<T> Mut<T>
where
	T: Copy + SaturatingAddAssign,
{
	pub fn saturating_increment(&self, rhs: T) {
		let mut v = self.get();
		v.saturating_add_assign(rhs);
		self.set(v);
	}

	pub fn saturating_decrement(&self, rhs: T) {
		let mut v = self.get();
		v.saturating_sub_assign(rhs);
		self.set(v);
	}
}

impl<T> Display for Mut<T>
where
	T: Display + Copy,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.get())
	}
}

impl<T> PartialEq for Mut<T>
where
	T: PartialEq + Copy,
{
	fn eq(&self, other: &Self) -> bool {
		self.inner.get() == other.inner.get()
	}
}

//impl<T> PartialEq<T> for Mut<T>
//where
//	T: PartialEq + Copy,
//{
//	fn eq(&self, other: &T) -> bool {
//		&self.inner.get() == other
//	}
//}

impl<T> PartialOrd for Mut<T>
where
	T: PartialOrd + Copy,
{
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.inner.get().partial_cmp(&other.inner.get())
	}
}

impl<T> Ord for Mut<T>
where
	T: Ord + Copy,
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.inner.get().cmp(&other.inner.get())
	}
}

impl<T> Eq for Mut<T> where T: Eq + Copy {}

impl<T> fmt::Debug for Mut<T>
where
	T: fmt::Debug + Copy,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.get())
	}
}

impl<T> EguiInspect for Mut<T>
where
	T: EguiInspect + Copy,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.get().inspect(label, ui)
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		let mut v = self.get();
		v.inspect_mut(label, ui);
		self.set(v)
	}
}

impl<T> From<T> for Mut<T>
where
	T: Copy,
{
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

impl<T> PartialEq<T> for Mut<T>
where
	T: PartialEq + Copy,
{
	fn eq(&self, other: &T) -> bool {
		self.inner.get() == *other
	}
}

impl<T> Serialize for Mut<T>
where
	T: Copy + Serialize,
{
	fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.get().serialize(serializer)
	}
}

impl<'de, T> Deserialize<'de> for Mut<T>
where
	T: Copy + Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(Self {
			inner: Cell::new(Deserialize::deserialize(deserializer)?),
		})
	}
}

#[cfg(test)]
mod test {

	use super::*;

	struct Crab {}

	#[test]
	fn test_arena() {
		let position = Mut::new(42);
		position.set(12);
		position.add(7);
		position.mutate(|v| *v += 7);
		position.replace(|v| v + 2);

		println!("{position}")
	}
}
