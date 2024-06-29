/// `With` is automatically implemented for every (sized) type,
/// and provides a method `with` that applies a function. E.g.
///
///    Player::new().with(|p| p.position = start)
///
/// This avoids the need for explicit mutability like
///
///  let mut player = Player::new();
///  player.position = start;
///  player
///
pub trait With {
	fn with<F: FnOnce(&mut Self)>(self, f: F) -> Self;
}

impl<T> With for T {
	#[inline]
	fn with<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
		f(&mut self);
		self
	}
}

pub trait Apply {
	fn mutate<F: FnOnce(&mut Self)>(&mut self, f: F);
}

impl<T> Apply for T {
	#[inline]
	fn mutate<F: FnOnce(&mut Self)>(&mut self, f: F) {
		f(self)
	}
}
