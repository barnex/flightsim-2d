#![allow(non_camel_case_types)]

use bytemuck::{Pod, Zeroable};
use num_traits::AsPrimitive;
use num_traits::Float;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

mod dynamic;
mod inspect;
pub use dynamic::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct vec<T, const N: usize>(pub [T; N]);

pub type vec2<T> = vec<T, 2>;
pub type vec3<T> = vec<T, 3>;
pub type vec4<T> = vec<T, 4>;

pub type vec2f = vec2<f32>;
pub type vec3f = vec3<f32>;
pub type vec4f = vec4<f32>;

pub type vec2d = vec2<f64>;
pub type vec3d = vec3<f64>;
pub type vec4d = vec4<f64>;

pub type vec2i = vec2<i32>;
pub type vec3i = vec3<i32>;
pub type vec4i = vec4<i32>;

pub type vec2u = vec2<u32>;
pub type vec3u = vec3<u32>;
pub type vec4u = vec4<u32>;

pub type vec2u8 = vec2<u8>;
pub type vec3u8 = vec3<u8>;
pub type vec4u8 = vec4<u8>;

#[rustfmt::skip]
mod constructors {
	use super::*;

	#[inline(always)] pub const fn vec2<T>(x: T, y: T            ) -> vec2<T> { vec([x, y]) }
	#[inline(always)] pub const fn vec3<T>(x: T, y: T, z: T      ) -> vec3<T> { vec([x, y, z]) }
	#[inline(always)] pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> vec4<T> { vec([x, y, z, w]) }

	#[inline(always)] pub const fn vec2f(x: f32, y: f32                ) -> vec2f { vec([x, y]) }
	#[inline(always)] pub const fn vec3f(x: f32, y: f32, z: f32        ) -> vec3f { vec([x, y, z]) }
	#[inline(always)] pub const fn vec4f(x: f32, y: f32, z: f32, w: f32) -> vec4f { vec([x, y, z, w]) }

	#[inline(always)] pub const fn vec2d(x: f64, y: f64                ) -> vec2d { vec([x, y]) }
	#[inline(always)] pub const fn vec3d(x: f64, y: f64, z: f64        ) -> vec3d { vec([x, y, z]) }
	#[inline(always)] pub const fn vec4d(x: f64, y: f64, z: f64, w: f64) -> vec4d { vec([x, y, z, w]) }

	#[inline(always)] pub const fn vec2i(x: i32, y: i32                ) -> vec2i { vec([x, y]) }
	#[inline(always)] pub const fn vec3i(x: i32, y: i32, z: i32        ) -> vec3i { vec([x, y, z]) }
	#[inline(always)] pub const fn vec4i(x: i32, y: i32, z: i32, w: i32) -> vec4i { vec([x, y, z, w]) }

	#[inline(always)] pub const fn vec2u(x: u32, y: u32                ) -> vec2u { vec([x, y]) }
	#[inline(always)] pub const fn vec3u(x: u32, y: u32, z: u32        ) -> vec3u { vec([x, y, z]) }
	#[inline(always)] pub const fn vec4u(x: u32, y: u32, z: u32, w: u32) -> vec4u { vec([x, y, z, w]) }

	impl<T, const N: usize> Default for vec<T, N>
	where
		[T; N]: Default,
	{
		fn default() -> Self {
			Self(<[T; N]>::default())
		}
	}

	impl<T: Copy, const N: usize> vec<T, N> {
		pub const fn splat(v: T) -> Self {
			Self([v; N])
		}
	}
}
pub use constructors::*;

#[rustfmt::skip]
mod constants {
	use super::*;

	pub trait Number: Copy{
		const ZERO: Self;
		const ONE: Self;
	}

	impl Number for f32 { const ONE: Self = 1.0; const ZERO: Self = 0.0; }
	impl Number for f64 { const ONE: Self = 1.0; const ZERO: Self = 0.0; }
	impl Number for u8  { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for u16 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for u32 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for u64 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i8  { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i16 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i32 { const ONE: Self = 1; const ZERO: Self = 0; }
	impl Number for i64 { const ONE: Self = 1; const ZERO: Self = 0; }

	impl<T: Number, const N: usize> vec<T, N> { 
		pub const ZERO: Self = Self::splat(T::ZERO); 
		pub const ONES: Self = Self::splat(T::ONE); 
	}

	impl<T: Number> vec<T, 2> { 
		pub const EX: Self = Self([T::ONE, T::ZERO]);
		pub const EY: Self = Self([T::ZERO, T::ONE]);
	}
	impl<T: Number> vec<T, 3> { 
		pub const EX: Self = Self([T::ONE, T::ZERO, T::ZERO]);
		pub const EY: Self = Self([T::ZERO, T::ONE, T::ZERO]);
		pub const EZ: Self = Self([T::ZERO, T::ZERO, T::ONE]);
	}
	impl<T: Number> vec<T, 4> { 
		pub const EX: Self = Self([T::ONE, T::ZERO, T::ZERO, T::ZERO]);
		pub const EY: Self = Self([T::ZERO, T::ONE, T::ZERO, T::ZERO]);
		pub const EZ: Self = Self([T::ZERO, T::ZERO, T::ONE, T::ZERO]);
		pub const EW: Self = Self([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
	}

}
pub use constants::*;

#[rustfmt::skip]
mod twiddle {
	use super::*;

	impl<T: Copy> vec2<T> {
		#[inline(always)] pub const fn x(self) -> T { self.0[0] }
		#[inline(always)] pub const fn y(self) -> T { self.0[1] }
	}

	impl<T: Copy> vec3<T> {
		#[inline(always)] pub const fn x(self) -> T { self.0[0] }
		#[inline(always)] pub const fn y(self) -> T { self.0[1] }
		#[inline(always)] pub const fn z(self) -> T { self.0[2] }

		#[inline(always)] pub const fn yz(self) -> vec2<T> { vec([self.0[1], self.0[2]]) }
		#[inline(always)] pub const fn xz(self) -> vec2<T> { vec([self.0[0], self.0[2]]) }
		#[inline(always)] pub const fn xy(self) -> vec2<T> { vec([self.0[0], self.0[1]]) }
	}

	impl<T: Copy> vec4<T> {
		#[inline(always)] pub const fn x(self) -> T { self.0[0] }
		#[inline(always)] pub const fn y(self) -> T { self.0[1] }
		#[inline(always)] pub const fn z(self) -> T { self.0[2] }
		#[inline(always)] pub const fn w(self) -> T { self.0[3] }

		#[inline(always)] pub const fn yz(self) -> vec2<T> { vec([self.0[1], self.0[2]]) }
		#[inline(always)] pub const fn xz(self) -> vec2<T> { vec([self.0[0], self.0[2]]) }
		#[inline(always)] pub const fn xy(self) -> vec2<T> { vec([self.0[0], self.0[1]]) }

		#[inline(always)] pub const fn xyz(self) -> vec3<T> { vec([self.0[0], self.0[1], self.0[2]]) }
	}

	impl<T> vec2<T> { #[inline(always)] pub fn append(self, z: T) -> vec3<T> { let vec([x, y]) = self;    vec([x, y, z]) } }
	impl<T> vec3<T> { #[inline(always)] pub fn append(self, w: T) -> vec4<T> { let vec([x, y, z]) = self; vec([x, y, z, w]) } }
}

#[rustfmt::skip]
mod conversions{
	use super::*;

	impl<T, const N: usize> From<[T; N]>    for vec<T, N> { fn from(inner: [T; N]) -> Self   { Self(inner) } }
	impl<T, const N: usize> From<vec<T, N>> for [T; N]    { fn from(v: vec<T, N>)  -> [T; N] { v.0 } }

	impl<T> From<(T, T)>       for vec2<T> { #[inline(always)] fn from((x, y):       (T, T))       -> Self { Self([x, y])       } }
	impl<T> From<(T, T, T)>    for vec3<T> { #[inline(always)] fn from((x, y, z):    (T, T, T))    -> Self { Self([x, y, z])    } }
	impl<T> From<(T, T, T, T)> for vec4<T> { #[inline(always)] fn from((x, y, z, w): (T, T, T, T)) -> Self { Self([x, y, z, w]) } }

	impl<T> From<vec2<T>> for (T, T)       { #[inline(always)] fn from(vec([x, y]): vec2<T>)       -> (T, T)       { (x, y)       } }
	impl<T> From<vec3<T>> for (T, T, T)    { #[inline(always)] fn from(vec([x, y, z]): vec3<T>)    -> (T, T, T)    { (x, y, z)    } }
	impl<T> From<vec4<T>> for (T, T, T, T) { #[inline(always)] fn from(vec([x, y, z, w]): vec4<T>) -> (T, T, T, T) { (x, y, z, w) } }

	impl<T: AsPrimitive<f32>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_f32(self) -> vec<f32, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<f64>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_f64(self) -> vec<f64, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i64>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_i64(self) -> vec<i64, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i32>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_i32(self) -> vec<i32, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i16>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_i16(self) -> vec<i16, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<i8 >, const N: usize> vec<T, N> { #[inline(always)] pub fn as_i8 (self) -> vec<i8 , N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u64>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_u64(self) -> vec<u64, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u32>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_u32(self) -> vec<u32, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u16>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_u16(self) -> vec<u16, N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<u8 >, const N: usize> vec<T, N> { #[inline(always)] pub fn as_u8 (self) -> vec<u8 , N> { self.map(|v| v.as_()) } }
	impl<T: AsPrimitive<usize>, const N: usize> vec<T, N> { #[inline(always)] pub fn as_usize(self) -> vec<usize, N> { self.map(|v| v.as_()) } }
}

impl<T, const N: usize> std::fmt::Debug for vec<T, N>
where
	T: std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.iter()).finish()
	}
}

impl<T, const N: usize> std::fmt::Display for vec<T, N>
where
	T: std::fmt::Display + std::fmt::Debug,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.iter()).finish()
	}
}

// TODO: remove, use each_mut instead:
// https://doc.rust-lang.org/std/primitive.array.html#method.each_mut
pub trait SupportedSize<const N: usize> {
	fn zip<T, U>(a: vec<T, N>, b: vec<U, N>) -> vec<(T, U), N>;
	fn zip_mut<T, U, F: Fn(&mut T, U)>(a: &mut vec<T, N>, b: vec<U, N>, f: F);
	fn reduce<T, F: Fn(T, T) -> T>(a: vec<T, N>, f: F) -> T;

	#[inline(always)]
	fn for_each<T, F: Fn(&mut T)>(a: &mut vec<T, N>, f: F) {
		Self::zip_mut(a, vec::splat(()), |ptr, ()| f(ptr))
	}

	#[inline(always)]
	fn zip_map<T, U, V, F: Fn(T, U) -> V>(a: vec<T, N>, b: vec<U, N>, f: F) -> vec<V, N> {
		Self::zip(a, b).map(|(a, b)| f(a, b))
	}
}

impl SupportedSize<2> for () {
	#[inline(always)]
	fn zip<T, U>(vec([a0, a1]): vec<T, 2>, vec([b0, b1]): vec<U, 2>) -> vec<(T, U), 2> {
		vec([(a0, b0), (a1, b1)])
	}

	#[inline(always)]
	fn zip_mut<T, U, F: Fn(&mut T, U)>(a: &mut vec<T, 2>, vec([b0, b1]): vec<U, 2>, f: F) {
		f(&mut a[0], b0);
		f(&mut a[1], b1);
	}

	#[inline(always)]
	fn reduce<T, F: Fn(T, T) -> T>(vec([a0, a1]): vec<T, 2>, f: F) -> T {
		f(a0, a1)
	}
}

impl SupportedSize<3> for () {
	#[inline(always)]
	fn zip<T, U>(vec([a0, a1, a2]): vec<T, 3>, vec([b0, b1, b2]): vec<U, 3>) -> vec<(T, U), 3> {
		vec([(a0, b0), (a1, b1), (a2, b2)])
	}

	#[inline(always)]
	fn zip_mut<T, U, F: Fn(&mut T, U)>(a: &mut vec<T, 3>, vec([b0, b1, b2]): vec<U, 3>, f: F) {
		f(&mut a[0], b0);
		f(&mut a[1], b1);
		f(&mut a[2], b2);
	}

	#[inline(always)]
	fn reduce<T, F: Fn(T, T) -> T>(vec([a0, a1, a2]): vec<T, 3>, f: F) -> T {
		f(f(a0, a1), a2)
	}
}

impl SupportedSize<4> for () {
	#[inline(always)]
	fn zip<T, U>(vec([a0, a1, a2, a3]): vec<T, 4>, vec([b0, b1, b2, b3]): vec<U, 4>) -> vec<(T, U), 4> {
		vec([(a0, b0), (a1, b1), (a2, b2), (a3, b3)])
	}

	#[inline(always)]
	fn zip_mut<T, U, F: Fn(&mut T, U)>(a: &mut vec<T, 4>, vec([b0, b1, b2, b3]): vec<U, 4>, f: F) {
		f(&mut a[0], b0);
		f(&mut a[1], b1);
		f(&mut a[2], b2);
		f(&mut a[3], b3);
	}

	#[inline(always)]
	fn reduce<T, F: Fn(T, T) -> T>(vec([a0, a1, a2, a3]): vec<T, 4>, f: F) -> T {
		f(f(a0, a1), f(a2, a3))
	}
}

mod iterator {
	use super::*;

	impl<T, const N: usize> vec<T, N> {
		#[inline]
		pub fn iter(self) -> impl Iterator<Item = T> {
			self.0.into_iter()
		}

		#[inline]
		pub fn map<U, F: Fn(T) -> U>(self, f: F) -> vec<U, N> {
			vec(self.0.map(f))
		}
	}

	impl<T, const N: usize> vec<T, N>
	where
		(): SupportedSize<N>,
	{
		#[inline]
		pub fn all<F: Fn(T) -> bool>(self, f: F) -> bool {
			self.map(f).reduce(|a, b| a && b)
		}

		#[inline]
		pub fn any<F: Fn(T) -> bool>(self, f: F) -> bool {
			self.map(f).reduce(|a, b| a || b)
		}

		#[inline]
		pub fn reduce<F: Fn(T, T) -> T>(self, f: F) -> T {
			<()>::reduce(self, f)
		}

		#[inline]
		pub fn zip<U>(self, rhs: vec<U, N>) -> vec<(T, U), N> {
			<()>::zip(self, rhs)
		}

		#[inline]
		pub fn zip_with<U, V, F: Fn(T, U) -> V>(self, rhs: vec<U, N>, f: F) -> vec<V, N> {
			<()>::zip_map(self, rhs, f)
		}
	}
	impl<T, const N: usize> vec<T, N>
	where
		(): SupportedSize<N>,
		T: Add<Output = T>,
	{
		#[inline]
		pub fn sum(self) -> T {
			self.reduce(T::add)
		}
	}
}

/// The indexing operator `[]`.
/// ```
/// # use vector::*;
/// assert_eq!(vec3(1,2,3)[0], 1);
/// assert_eq!(vec3(1,2,3)[1], 2);
/// ```
impl<T, const N: usize> Index<usize> for vec<T, N> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

/// The indexing operator `[]`.
/// ```
/// # use vector::*;
/// let mut v = vec2(0, 0);
/// v[1] = 42;
/// assert_eq!(v, vec2(0, 42))
/// ```
impl<T, const N: usize> IndexMut<usize> for vec<T, N> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.0[index]
	}
}

/// The addition operator `+`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) + vec2(3,4), vec2(4,6));
/// assert_eq!(vec3u(1,2,3) + vec3u(4,5,6), vec3u(5,7,9));
/// ```
impl<T, const N: usize> Add for vec<T, N>
where
	(): SupportedSize<N>,
	T: Add<Output = T>,
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		<()>::zip_map(self, rhs, T::add)
	}
}

/// Vector + constant adds the constant to each component.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) + 3, vec2(4,5));
/// assert_eq!(vec3u(1,2,3) + 1, vec3u(2,3,4));
/// ```
impl<T, const N: usize> Add<T> for vec<T, N>
where
	(): SupportedSize<N>,
	T: Add<Output = T> + Copy,
{
	type Output = Self;

	fn add(self, rhs: T) -> Self::Output {
		self + vec::splat(rhs)
	}
}

/// The addition assignment operator `+=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v += vec2(1, 2);
/// assert_eq!(v, vec2(4,6));
/// ```
impl<T, const N: usize> AddAssign for vec<T, N>
where
	(): SupportedSize<N>,
	T: AddAssign,
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Self) {
		<()>::zip_mut(self, rhs, T::add_assign)
	}
}

/// The division operator `/`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(10.0, 20.0) / vec2(2.0, 5.0), vec2(5.0, 4.0));
/// ```
impl<T, const N: usize> Div for vec<T, N>
where
	(): SupportedSize<N>,
	T: Div<Output = T>,
{
	type Output = vec<T, N>;

	#[inline(always)]
	fn div(self, rhs: Self) -> Self::Output {
		<()>::zip_map(self, rhs, T::div)
	}
}

/// The division operator `/`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(10.0, 20.0) / 2.0, vec2(5.0, 10.0));
/// ```
impl<T, const N: usize> Div<T> for vec<T, N>
where
	(): SupportedSize<N>,
	T: Div<Output = T> + Copy,
{
	type Output = vec<T, N>;

	#[inline(always)]
	fn div(self, rhs: T) -> Self::Output {
		self.map(|v| v / rhs)
	}
}

/// The division assignment operator `/=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(30, 50);
/// v /= vec2(3, 2);
/// assert_eq!(v, vec2(10, 25));
/// ```
impl<T, const N: usize> DivAssign for vec<T, N>
where
	(): SupportedSize<N>,
	T: DivAssign,
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: Self) {
		<()>::zip_mut(self, rhs, T::div_assign)
	}
}

/// The division assignment operator `/=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(30, 50);
/// v /= 2;
/// assert_eq!(v, vec2(15, 25));
/// ```
impl<T, const N: usize> DivAssign<T> for vec<T, N>
where
	(): SupportedSize<N>,
	T: DivAssign + Copy,
{
	#[inline(always)]
	fn div_assign(&mut self, rhs: T) {
		<()>::for_each(self, |v| *v /= rhs)
	}
}

/// The multiplication operator `*`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) * vec2(3,4), vec2(3,8));
/// assert_eq!(vec3u(1,2,3) * vec3u(4,5,6), vec3u(4,10,18));
/// ```
impl<T, const N: usize> Mul<Self> for vec<T, N>
where
	(): SupportedSize<N>,
	T: Mul<Output = T>,
{
	type Output = vec<T, N>;

	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output {
		<()>::zip_map(self, rhs, T::mul)
	}
}

/// The multiplication operator `*`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(1,2) * 2, vec2(2,4));
/// assert_eq!(vec3u(1,2,3) * 4, vec3u(4,8,12));
/// ```
impl<T, const N: usize> Mul<T> for vec<T, N>
where
	(): SupportedSize<N>,
	T: Mul<Output = T> + Copy,
{
	type Output = vec<T, N>;

	#[inline(always)]
	fn mul(self, rhs: T) -> Self::Output {
		self.map(|v| v * rhs)
	}
}

#[rustfmt::skip]
mod mul_primitive{
	use super::*;

/// The multiplication operator `*`.
/// ```
/// # use vector::*;
/// assert_eq!(2.0 * vec2(1.0,2.0), vec2(2.0, 4.0));
/// ```
impl<const N: usize> Mul<vec<f64, N>> for f64 { type Output = vec<f64, N>; #[inline(always)] fn mul(self, rhs: vec<f64, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<f32, N>> for f32 { type Output = vec<f32, N>; #[inline(always)] fn mul(self, rhs: vec<f32, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<u64, N>> for u64 { type Output = vec<u64, N>; #[inline(always)] fn mul(self, rhs: vec<u64, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<u32, N>> for u32 { type Output = vec<u32, N>; #[inline(always)] fn mul(self, rhs: vec<u32, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<u16, N>> for u16 { type Output = vec<u16, N>; #[inline(always)] fn mul(self, rhs: vec<u16, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<u8 , N>> for u8  { type Output = vec<u8 , N>; #[inline(always)] fn mul(self, rhs: vec<u8 , N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<i64, N>> for i64 { type Output = vec<i64, N>; #[inline(always)] fn mul(self, rhs: vec<i64, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<i32, N>> for i32 { type Output = vec<i32, N>; #[inline(always)] fn mul(self, rhs: vec<i32, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<i16, N>> for i16 { type Output = vec<i16, N>; #[inline(always)] fn mul(self, rhs: vec<i16, N>) -> Self::Output { rhs.map(|v| self * v) } }
impl<const N: usize> Mul<vec<i8 , N>> for i8  { type Output = vec<i8 , N>; #[inline(always)] fn mul(self, rhs: vec<i8 , N>) -> Self::Output { rhs.map(|v| self * v) } }
}

/// The multiplication assignment operator `*=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v *= vec2(1, 2);
/// assert_eq!(v, vec2(3,8));
/// ```
impl<T, const N: usize> MulAssign for vec<T, N>
where
	(): SupportedSize<N>,
	T: MulAssign,
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: Self) {
		<()>::zip_mut(self, rhs, T::mul_assign)
	}
}

/// The multiplication assignment operator `*=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 4);
/// v *= 3;
/// assert_eq!(v, vec2(9, 12));
/// ```
impl<T, const N: usize> MulAssign<T> for vec<T, N>
where
	(): SupportedSize<N>,
	T: MulAssign + Copy,
{
	#[inline(always)]
	fn mul_assign(&mut self, rhs: T) {
		<()>::for_each(self, |v| *v *= rhs)
	}
}

/// The unary negation operator `-`.
/// ```
/// # use vector::*;
/// assert_eq!(-vec2i(1, 2), vec2i(-1, -2));
/// ```
impl<T, const N: usize> Neg for vec<T, N>
where
	(): SupportedSize<N>,
	T: Neg<Output = T>,
{
	type Output = vec<T, N>;

	#[inline(always)]
	fn neg(self) -> Self::Output {
		self.map(T::neg)
	}
}

/// The subtraction operator `-`.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(10,20) - vec2(1,2), vec2(9,18));
/// assert_eq!(vec3u(4,5,6) - vec3u(1,2,3), vec3u(3,3,3));
/// ```
impl<T, const N: usize> Sub for vec<T, N>
where
	(): SupportedSize<N>,
	T: Sub<Output = T>,
{
	type Output = vec<T, N>;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output {
		<()>::zip_map(self, rhs, T::sub)
	}
}

/// Vector - constant subtracts the constant from each component.
/// ```
/// # use vector::*;
/// assert_eq!(vec2(4,5) - 3, vec2(1,2));
/// assert_eq!(vec3u(1,2,3) - 1, vec3u(0,1,2));
/// ```
impl<T, const N: usize> Sub<T> for vec<T, N>
where
	(): SupportedSize<N>,
	T: Sub<Output = T> + Copy,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		self - vec::splat(rhs)
	}
}

/// The addition assignment operator `+=`.
/// ```
/// # use vector::*;
/// let mut v = vec2(3, 5);
/// v -= vec2(1, 2);
/// assert_eq!(v, vec2(2, 3));
/// ```
impl<T, const N: usize> SubAssign for vec<T, N>
where
	(): SupportedSize<N>,
	T: SubAssign,
{
	#[inline(always)]
	fn sub_assign(&mut self, rhs: Self) {
		<()>::zip_mut(self, rhs, T::sub_assign)
	}
}

impl<T, const N: usize> vec<T, N>
where
	(): SupportedSize<N>,
	T: Add<Output = T> + Mul<Output = T> + Copy,
{
	/// The dot (inner) product of two vectors.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec2(2, 3).dot(vec2(4, 5)), 23);
	/// assert_eq!(vec3(1, 2, 3).dot(vec3(0, 0, 4)), 12);
	/// assert_eq!(vec4(1, 2, 3, 4).dot(vec4(0, 1, 0, 0)), 2);
	/// ```
	#[inline]
	pub fn dot(self, rhs: Self) -> T {
		(self * rhs).sum()
	}

	/// Length squared.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec2(3, 4).len2(), 25);
	/// ```
	#[inline]
	pub fn len2(self) -> T {
		self.dot(self)
	}
}

impl<T, const N: usize> vec<T, N>
where
	(): SupportedSize<N>,
	T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
	/// Distance between two points, squared
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec2i(3, 4).distance_squared(vec2i(2, 4)), 1);
	/// ```
	#[inline]
	pub fn distance_squared(self, rhs: Self) -> T {
		(self - rhs).len2()
	}
}

impl<T, const N: usize> vec<T, N>
where
	(): SupportedSize<N>,
	T: Float,
{
	/// Length (norm) of a vector.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec2f(3.0, 4.0).len(), 5.0);
	/// assert_eq!(vec3d(0.0, 3.0, 4.0).len(), 5.0);
	/// ```
	#[inline]
	pub fn len(self) -> T {
		self.len2().sqrt()
	}

	/// Distance between two points.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec2f(3.0, 4.0).distance_to(vec2f(2.0, 4.0)), 1.0);
	/// ```
	#[inline]
	pub fn distance_to(self, rhs: Self) -> T {
		(self - rhs).len()
	}

	/// Vector with same direction but length normalized to 1,
	/// unless length was zero.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec3(0.0, 3.0, 4.0).normalized(), vec3(0.0, 3.0, 4.0) / 5.0);
	/// assert_eq!(vec2(0.0, 0.0).normalized(), vec2(0.0, 0.0));
	/// ```
	#[inline]
	pub fn normalized(self) -> Self {
		let len = self.len();
		if len == T::zero() {
			self
		} else {
			self / len
		}
	}
}

impl<T> vec3<T>
where
	T: Copy + Mul<T, Output = T> + Sub<T, Output = T>,
{
	/// Cross product.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec3(1,0,0).cross(vec3(0,1,0)), vec3(0,0,1));
	/// ```
	#[inline]
	pub fn cross(self, rhs: Self) -> Self {
		Self([
			self.y() * rhs.z() - self.z() * rhs.y(),
			self.z() * rhs.x() - self.x() * rhs.z(),
			self.x() * rhs.y() - self.y() * rhs.x(),
		])
	}
}

impl<T> vec2<T>
where
	T: Copy + Mul<T, Output = T> + Sub<T, Output = T> + Number,
{
	/// Cross product.
	/// ```
	/// # use vector::*;
	/// assert_eq!(vec2(1,0).cross(vec2(0,1)), 1);
	/// ```
	#[inline]
	pub fn cross(self, rhs: Self) -> T {
		self.append(T::ZERO).cross(rhs.append(T::ZERO)).z()
	}
}

impl<const N: usize> vec<f32, N>
where
	(): SupportedSize<N>,
{
	#[inline]
	pub fn floor(self) -> vec<i32, N> {
		self.map(|v| v.floor() as i32)
	}

	#[inline]
	pub fn round(self) -> vec<i32, N> {
		self.map(|v| v.round() as i32)
	}
}

// As of 2024, serde does not support [T;N] for arbitrary N (https://github.com/serde-rs/serde/issues/1937).
// When supported, these impls can be replaced by #[derive].
impl<T, const N: usize> Serialize for vec<T, N>
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

impl<'de, T, const N: usize> Deserialize<'de> for vec<T, N>
where
	[T; N]: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(Self(Deserialize::deserialize(deserializer)?))
	}
}

unsafe impl<T, const N: usize> Zeroable for vec<T, N> where T: Pod + Zeroable {}
unsafe impl<T, const N: usize> Pod for vec<T, N> where T: Pod + Zeroable {}
