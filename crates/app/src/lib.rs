#![allow(unused_imports)]
#![allow(clippy::match_like_matches_macro)]
#![allow(clippy::unit_arg)]
#![allow(clippy::type_complexity)]
#![allow(clippy::useless_conversion)]

mod prelude;
mod profiler;

mod ai;
mod app;
mod cells;
mod core_game;
mod renderer;
mod scope;
mod util;

pub use app::App;

pub(crate) use ai::*;
pub(crate) use cells::*;
pub(crate) use core_game::*;
pub(crate) use renderer::*;
pub(crate) use scope::*;
pub(crate) use util::*;

pub fn select<T>(true_value: T, false_value: T, cond: bool) -> T {
	if cond {
		true_value
	} else {
		false_value
	}
}

pub fn flip(v: &mut bool) {
	*v = !*v;
}

pub fn clamp_u32(v: i32) -> u32 {
	v.max(0).try_into().unwrap()
}

use crate::prelude::*;
pub trait Center {
	fn center(self) -> crate::vec2f;
}

impl Center for vec2i {
	fn center(self) -> crate::vec2f {
		self.as_f32() + 0.5
	}
}
