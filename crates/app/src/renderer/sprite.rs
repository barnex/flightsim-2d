use crate::prelude::*;

// NOTE: Default only needed for Vec<Sprite>: EguiInspect
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Sprite(pub u8);

impl Sprite {
	pub const RECTANGLE: Self = Self(0);
	pub const FERRIS: Self = Self(1);
	pub const FERRIS_SMILE: Self = Self(2);
	pub const FERRIS_UNSAFE: Self = Self(3);
	pub const FERRIS_PADDLE: Self = Self(4);
	pub const LEAF: Self = Self(17);
	pub const PLANE: Self = Self(33);
	pub const WING: Self = Self(34);
}

impl EguiInspect for Sprite {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.0.inspect(label, ui);
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.0.inspect_mut(label, ui);
	}
}
