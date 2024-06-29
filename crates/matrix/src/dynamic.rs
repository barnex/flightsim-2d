use std::ops::AddAssign;

use crate::*;
use vector::*;

pub struct Matrix<T> {
	pub size: [u32; 2], // rows, cols
	pub el: Box<[T]>,
}

impl<T> Matrix<T>
where
	T: Default + Clone,
{
	pub fn new([rows, cols]: [u32; 2]) -> Self {
		Self {
			size: [rows, cols],
			el: vec![T::default(); (rows * cols) as usize].into(),
		}
	}
}

impl<T> Matrix<T> {
	pub fn rows(&self) -> u32 {
		self.size[0]
	}

	pub fn cols(&self) -> u32 {
		self.size[1]
	}

	pub fn row(&self, row: u32) -> &[T] {
		let cols = self.cols();
		&self.el[(row * cols) as usize..((row + 1) * cols) as usize]
	}
}

impl<T> Matrix<T>
where
	T: Default + Copy + Mul<T, Output = T> + AddAssign,
{
	pub fn mul_vec(dst: &mut Vector<T>, matrix: &Self, rhs: Vector<T>) {
		dst.clear();
		let [rows, cols] = matrix.size;
		for i in 0..rows {
			for j in 0..cols {
				dst[i as usize] += matrix[i][j as usize] * rhs[j as usize];
			}
		}
	}
}

impl<T> Index<u32> for Matrix<T> {
	type Output = [T];

	fn index(&self, index: u32) -> &Self::Output {
		self.row(index)
	}
}
