use crate::prelude::*;

#[derive(Default)]
pub struct Commands(RefCell<Vec<Box<dyn FnOnce(&mut GameState)>>>);

impl fmt::Debug for Commands {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("Commands").field(&self.0.borrow().len()).finish()
	}
}

impl Commands {
	pub fn push<F: FnOnce(&mut GameState) + 'static>(&self, f: F) {
		self.0.borrow_mut().push(Box::new(f))
	}

	pub fn push_maybe<F: FnOnce(&mut GameState) -> Option<()> + 'static>(&self, f: F) {
		self.0.borrow_mut().push(Box::new(move |gs| {
			let _ = f(gs);
		}))
	}
}

impl GameState {
	pub fn exec_command_queue(&mut self) {
		let commands = std::mem::take(&mut *self.commands.0.borrow_mut());
		for cmd in commands {
			cmd(self)
		}
	}
}
