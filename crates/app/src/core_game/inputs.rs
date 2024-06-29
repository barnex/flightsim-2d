use crate::*;

type Key = egui::Key;

#[derive(Serialize, Deserialize, Debug, EguiInspect)]
pub struct Inputs {
	pub pointer_pos_pix: vec2f,
	pub scroll_delta: vec2f,
	pub zoom_delta: f32,

	pub mouse_is_down: bool,
	pub mouse_just_pressed: bool,
	pub mouse_just_released: bool,

	#[inspect(hide)]
	pub keys_down: HashSet<Key>,
}

impl Inputs {
	pub fn record(&mut self, pointer_offset: (f32, f32), inputs: &egui::InputState) {
		if let Some(pointer_pos) = inputs.pointer.interact_pos() {
			self.pointer_pos_pix = vec2f(pointer_pos.x, pointer_pos.y) - vec2f::from(pointer_offset);
		}

		self.mouse_is_down = inputs.pointer.button_down(egui::PointerButton::Primary);
		self.mouse_just_pressed = inputs.pointer.button_pressed(egui::PointerButton::Primary);
		self.mouse_just_released = inputs.pointer.button_released(egui::PointerButton::Primary);

		self.zoom_delta *= inputs.zoom_delta();
		self.scroll_delta += vec2(inputs.raw_scroll_delta.x, inputs.raw_scroll_delta.y);

		self.keys_down.clear();
		self.keys_down.extend(&inputs.keys_down);
	}

	// to be called after each frame
	pub fn reset(&mut self) {
		self.scroll_delta = vec::ZERO;
		self.zoom_delta = 1.0;
		self.mouse_just_pressed = false;
		self.mouse_just_released = false;
	}
}

impl Default for Inputs {
	fn default() -> Self {
		Self {
			pointer_pos_pix: default(),
			scroll_delta: default(),
			zoom_delta: 1.0,
			mouse_is_down: default(),
			mouse_just_pressed: default(),
			mouse_just_released: default(),
			keys_down: default(),
		}
	}
}
