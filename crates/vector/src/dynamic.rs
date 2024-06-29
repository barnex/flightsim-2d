use std::ops::{Deref, DerefMut};

pub struct Vector<T>(pub Box<[T]>);

impl<T> Vector<T>
where
	T: Default + Copy,
{
	pub fn new(len: u32) -> Self {
		Self(vec![T::default(); len as usize].into())
	}

	#[inline(always)]
	pub fn clear(&mut self) {
		let zero = T::default();
		for v in self.iter_mut() {
			*v = zero;
		}
	}
}

impl<T> Deref for Vector<T> {
	type Target = [T];

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for Vector<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

// impl<T> Index<u32> for Vector<T> {
// 	type Output = T;
//
// 	#[inline(always)]
// 	fn index(&self, index: u32) -> &Self::Output {
// 		&self.0[index as usize]
// 	}
// }
