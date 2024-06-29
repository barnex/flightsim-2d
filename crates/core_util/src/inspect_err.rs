use std::fmt;

/// Extension trait providing `inspect_err`, to inspect/log errors inline.
pub trait InspectErr: Sized {
	type Error: fmt::Debug;

	/// Call `f(err)` if `self` is an error, passthrough `self` unchanged.
	/// Intended to inspect/log errors. E.g.:
	///
	/// File::open("foo").inspect_err(|e|log::warning("ignoring file error {e:?}"));
	fn inspect_err<F: FnOnce(&Self::Error)>(self, f: F) -> Self;

	/// Log error if `self` is an error, passthrough `self` unchanged
	fn log_err(self) -> Self {
		self.inspect_err(|e| log::error!("{e:#?}"))
	}
}

impl<T, E> InspectErr for Result<T, E>
where
	E: fmt::Debug,
{
	type Error = E;
	#[inline]
	fn inspect_err<F: FnOnce(&Self::Error)>(self, f: F) -> Self {
		if let Err(e) = &self {
			f(e)
		}
		self
	}
}
