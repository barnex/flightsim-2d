pub use crate::*;

pub use core_util::*;
pub use matrix::*;
pub use proc_macros::*;
pub use vector::*;

pub use anyhow::{anyhow, bail, Context, Error, Result};
pub use base64::prelude::*;
pub use bytemuck::{Pod, Zeroable};
pub use eframe::{egui_wgpu, egui_wgpu::CallbackTrait, wgpu, wgpu::util::DeviceExt as _};
pub use egui::{Rect, Ui};
pub use image::{DynamicImage, GenericImageView};
pub use itertools::Itertools;
pub use num_traits::AsPrimitive;
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use egui_inspect::{inspect_debug, EguiInspect};
pub use flate2::read::GzDecoder;
pub use flate2::write::GzEncoder;
pub use web_time::{Duration, Instant, SystemTime};

// AHash hashmap compatible with bevy_reflect::Reflect.
pub type HashMap<K, V> = std::collections::HashMap<K, V, std::hash::BuildHasherDefault<ahash::AHasher>>;
pub type HashSet<T> = std::collections::HashSet<T, std::hash::BuildHasherDefault<ahash::AHasher>>;

pub use std::cell::{Cell, RefCell};
pub use std::cmp::{PartialEq, PartialOrd};
pub use std::collections::BinaryHeap;
pub use std::collections::VecDeque;
pub use std::f32::consts::PI;
pub use std::fmt;
pub use std::hash::Hash;
pub use std::io;
pub use std::io::{Read, Write};
pub use std::iter;
pub use std::marker::PhantomData;
pub use std::mem;
pub use std::num::Saturating;
pub use std::num::{NonZeroU32, NonZeroU8};
pub use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Range, Sub, SubAssign};
pub use std::rc::Rc;
pub use std::sync::{Arc, Mutex, OnceLock};

pub type Pos = vec2i;

/// 1 degree in radians.
pub const DEG: f32 = PI / 180.0;

#[inline]
pub fn default<T: Default>() -> T {
	T::default()
}
