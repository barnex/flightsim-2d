#![allow(non_camel_case_types)]

mod dynamic;
mod inspect;
mod transforms;
pub use dynamic::*;
pub use transforms::*;

use bytemuck::{Pod, Zeroable};
use std::ops::{Add, Index, IndexMut, Mul};
use vector::vec;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct mat<T, const N: usize>(pub [[T; N]; N]);


impl<T, const N: usize> Default for mat<T, N>
where
	[[T; N]; N]: Default,
{
	fn default() -> Self {
		Self(<[[T; N]; N]>::default())
	}
}

pub type mat2x2<T> = mat<T, 2>;
pub type mat3x3<T> = mat<T, 3>;
pub type mat4x4<T> = mat<T, 4>;

pub type mat2x2f = mat<f32, 2>;
pub type mat3x3f = mat<f32, 3>;
pub type mat4x4f = mat<f32, 4>;

impl<T, const N: usize> Index<usize> for mat<T, N> {
	type Output = [T; N];

	fn index(&self, index: usize) -> &[T; N] {
		&self.0[index]
	}
}

impl<T, const N: usize> IndexMut<usize> for mat<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

impl<T, const N: usize> From<[[T; N]; N]> for mat<T, N> {
	#[inline(always)]
	fn from(value: [[T; N]; N]) -> Self {
		Self(value)
	}
}

impl<T, const N: usize> Mul<&mat<T, N>> for &mat<T, N>
where
	mat<T, N>: Default,
	T: Mul<Output = T> + Add<Output = T> + Copy,
{
	type Output = mat<T, N>;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: &mat<T, N>) -> Self::Output {
		let mut c: mat<T, N> = Default::default();
		for i in 0..N {
			for j in 0..N {
				for k in 0..N {
					c[i][j] = c[i][j] + rhs[i][k] * self[k][j]
				}
			}
		}
		c
	}
}

impl<T, const N: usize> Mul<vec<T, N>> for &mat<T, N>
where
	vec<T, N>: Default,
	T: Mul<Output = T> + Add<Output = T> + Copy,
{
	type Output = vec<T, N>;

	fn mul(self, rhs: vec<T, N>) -> Self::Output {
		let mut c: vec<T, N> = Default::default();
		for i in 0..N {
			for j in 0..N {
				c[i] = c[i] + self[i][j] * rhs[j];
			}
		}
		c
	}
}

impl<T, const N: usize> Mul<vec<T, N>> for mat<T, N>
where
	vec<T, N>: Default,
	T: Mul<Output = T> + Add<Output = T> + Copy,
{
	type Output = vec<T, N>;

	fn mul(self, rhs: vec<T, N>) -> Self::Output {
		(&self).mul(rhs)
	}
}

// allows chaining multiplications:  &a * &b * &c
impl<T, const N: usize> Mul<&mat<T, N>> for mat<T, N>
where
	mat<T, N>: Default,
	T: Mul<Output = T> + Add<Output = T> + Copy,
{
	type Output = mat<T, N>;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: &mat<T, N>) -> mat<T, N> {
		(&self).mul(rhs)
	}
}

impl<T, const N: usize> Mul<mat<T, N>> for mat<T, N>
where
	mat<T, N>: Default,
	T: Mul<Output = T> + Add<Output = T> + Copy,
{
	type Output = mat<T, N>;

	/// Matrix-Matrix multiplication.
	fn mul(self, rhs: mat<T, N>) -> mat<T, N> {
		(&self).mul(&rhs)
	}
}

impl<T> mat<T, 4>
where
	T: Copy,
{
	#[inline(always)]
	pub fn transpose(v: [[T; 4]; 4]) -> Self {
		Self([
			[v[0][0], v[1][0], v[2][0], v[3][0]],
			[v[0][1], v[1][1], v[2][1], v[3][1]],
			[v[0][2], v[1][2], v[2][2], v[3][2]],
			[v[0][3], v[1][3], v[2][3], v[3][3]],
		])
	}
}


impl<T> mat<T, 2>
where
	T: vector::Number,
{
	pub const UNIT: Self = Self([
		//_
		[T::ONE, T::ZERO],
		[T::ZERO, T::ONE],
	]);
}

impl<T> mat<T, 3>
where
	T: vector::Number,
{
	pub const UNIT: Self = Self([
		//_
		[T::ONE, T::ZERO, T::ZERO],
		[T::ZERO, T::ONE, T::ZERO],
		[T::ZERO, T::ZERO, T::ONE],
	]);
}

impl<T> mat<T, 4>
where
	T: vector::Number,
{
	pub const UNIT: Self = Self([
		//_
		[T::ONE, T::ZERO, T::ZERO, T::ZERO],
		[T::ZERO, T::ONE, T::ZERO, T::ZERO],
		[T::ZERO, T::ZERO, T::ONE, T::ZERO],
		[T::ZERO, T::ZERO, T::ZERO, T::ONE],
	]);
}

// As of 2024, serde does not support [T;N] for arbitrary N (https://github.com/serde-rs/serde/issues/1937).
// When supported, these impls can be replaced by #[derive].
impl<T, const N: usize> Serialize for mat<T, N>
where
	[T; N]: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

impl<'de, T, const N: usize> Deserialize<'de> for mat<T, N>
where
	[[T; N]; N]: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(Self(Deserialize::deserialize(deserializer)?))
	}
}


unsafe impl<T, const N: usize> Zeroable for mat<T, N> where T: Zeroable {}
unsafe impl<T, const N: usize> Pod for mat<T, N> where T: Pod {}