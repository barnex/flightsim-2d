use crate::prelude::*;

// NOTE: Default only needed for Vec<Sprite>: EguiInspect
#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default)]
pub struct Sprite {
	pub pos: vec2u8,
	pub size: vec2u8,
}

impl Sprite {
	pub const LEAF: Self = Self { pos: vec2(1, 0), size: vec2(1, 1) };
	pub const PLANE: Self = Self { pos: vec2(0, 5), size: vec2(8, 5) };
	pub const WING: Self = Self { pos: vec2(3, 0), size: vec2(2, 1) };
	pub const WHEEL: Self = Self { pos: vec2(5, 0), size: vec2(1, 1) };
	pub const CENTER: Self = Self { pos: vec2(6, 0), size: vec2(1, 1) };
}

impl EguiInspect for Sprite {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		(self.pos, self.size).inspect(label, ui);
	}
}
