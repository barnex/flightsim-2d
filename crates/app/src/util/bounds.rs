#![allow(unused)]
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bounds<T, const N: usize> {
	pub min: vec<T, N>,
	pub max: vec<T, N>,
}

impl<T: Clone, const N: usize> Bounds<T, N> {
	pub fn from_point(p: vec<T, N>) -> Self {
		Self { min: p.clone(), max: p }
	}
}

impl<T, const N: usize> Bounds<T, N>
where
	T: PartialOrd + Clone,
	(): SupportedSize<N>,
{
	#[must_use = "does not modify"]
	pub fn add(&self, p: vec<T, N>) -> Self {
		Self {
			min: self.min.clone().zip_with(p.clone(), partial_min),
			max: self.max.clone().zip_with(p.clone(), partial_max),
		}
	}
}

fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
	if a < b {
		a
	} else {
		b
	}
}

fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
	if a > b {
		a
	} else {
		b
	}
}

pub type Bounds2<T> = Bounds<T, 2>;
pub type Bounds3<T> = Bounds<T, 3>;

pub type Bounds2u = Bounds<u32, 2>;
pub type Bounds2i = Bounds<i32, 2>;
pub type Bounds2f = Bounds<f32, 2>;

impl<T: Copy> Bounds2<T> {
	pub fn x_range(self) -> Range<T> {
		self.min.x()..self.max.x()
	}

	pub fn y_range(self) -> Range<T> {
		self.min.y()..self.max.y()
	}
}

impl<T: AsPrimitive<i32>, const N: usize> Bounds<T, N> {
	pub fn as_i32(self) -> Bounds<i32, N> {
		self.map(|b| b.as_i32())
	}
}

impl<T: AsPrimitive<u32>, const N: usize> Bounds<T, N> {
	pub fn as_u32(self) -> Bounds<u32, N> {
		self.map(|b| b.as_u32())
	}
}

impl<T, const N: usize> Bounds<T, N> {
	pub fn map<U, F: Fn(vec<T, N>) -> vec<U, N>>(self, f: F) -> Bounds<U, N> {
		Bounds { min: f(self.min), max: f(self.max) }
	}
}

impl<T> Bounds2<T>
where
	T: Number + AddAssign,
	Range<T>: Iterator<Item = T> + Clone,
{
	pub fn iter_excl(self) -> impl Iterator<Item = vec2<T>> {
		self.x_range().cartesian_product(self.y_range()).map(vec::from)
	}

	//pub fn iter_incl(self) -> impl Iterator<Item = vec2<T>> {
	//	self.with(|slf| slf.max += vec2(T::ONE, T::ONE)).iter_excl()
	//}
}

impl<T, const N: usize> Bounds<T, N>
where
	T: Number + num_traits::PrimInt,
	vec<T, N>: Add<Output = vec<T, N>>,
{
	/// Add 1 to the `max` bound,
	/// equivalent to making the upper bound inclusive rather than exclusive.
	#[must_use]
	pub fn make_inclusive(self) -> Self {
		Self {
			min: self.min,
			max: self.max + vec::splat(T::ONE),
		}
	}
}

impl<T: PartialOrd, const N: usize> Bounds<T, N>
where
	(): SupportedSize<N>,
	T: Copy,
{
	pub fn contains(self, point: vec<T, N>) -> bool {
		self.min.zip(point).all(|(min, p)| p >= min) && self.max.zip(point).all(|(max, p)| p < max)
	}
}

impl<T, const N: usize> Bounds<T, N>
where
	T: PartialOrd,
	(): SupportedSize<N>,
{
	pub fn intersect(self, rhs: Self) -> Self {
		Self {
			min: self.min.zip_with(rhs.min, max),
			max: self.max.zip_with(rhs.max, min),
		}
	}
}

impl<T, const N: usize> Bounds<T, N>
where
	T: PartialOrd + Copy,
	(): SupportedSize<N>,
{
	pub fn intersects(self, rhs: Self) -> bool {
		!self.intersect(rhs).is_empty()
	}

	pub fn is_empty(&self) -> bool {
		!self.min.zip_with(self.max, |min, max| min < max).all(|v| v /* ==true*/)
	}
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
	if a <= b {
		a
	} else {
		b
	}
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
	if a >= b {
		a
	} else {
		b
	}
}

impl<T, const N: usize> Bounds<T, N>
where
	T: Copy,
	vec<T, N>: Add<Output = vec<T, N>> + Sub<Output = vec<T, N>>,
{
	/// Grow bounds by `radius` in all directions.
	#[must_use]
	pub fn with_margin(self, radius: T) -> Self {
		Self {
			min: self.min - vec::splat(radius),
			max: self.max + vec::splat(radius),
		}
	}

	#[must_use]
	pub fn around_point(p: vec<T, N>, radius: T) -> Self {
		Self::from_point(p).with_margin(radius)
	}
}
