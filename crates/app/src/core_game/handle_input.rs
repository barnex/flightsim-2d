use crate::*;

impl GameState {
	pub fn tick_inputs(&mut self) {
		self.mouse_pos = self.camera.screen_to_tile(self.inputs.pointer_pos_pix);
		self.handle_zoom();
		self.handle_scroll();
		self.handle_mouse();
		self.handle_keys();
		self.inputs.reset();
	}

	fn handle_keys(&mut self) {
		let mut pan_delta = vec2::ZERO;

		for k in &self.inputs.keys_down {
			use egui::Key;
			pan_delta += match k {
				Key::ArrowLeft | Key::S => vec2(-1, 0),
				Key::ArrowRight | Key::F => vec2(1, 0),
				Key::ArrowDown | Key::D => vec2(0, -1),
				Key::ArrowUp | Key::E => vec2(0, 1),
				_ => vec2::ZERO,
			}
		}

		// TODO: * dt for FPS-independent speed!
		//let sensitivity = 50.0 * self.dt; // <<<
		let sensitivity = 15.0;
		self.pan_camera(pan_delta.as_f32() * sensitivity);
	}

	fn handle_mouse(&mut self) {
		let pos = self.mouse_pos;

		// if self.inputs.mouse_just_pressed {
		// 	 self.dragging = Some((pos, pos))
		// }

		// if self.inputs.mouse_is_down {
		// 	if let Some((_start, end)) = &mut self.dragging {
		// 		*end = pos;
		// 	}
		// 	self.handle_mouse_drag(pos);
		// }

		if self.inputs.mouse_just_released {
			self.handle_mouse_up(self.mouse_pos);
		}
		//	match self.dragging.take() {
		//		None => (),
		//		Some(dragging) => self.handle_mouse_up(dragging),

		//	}
		//}
	}

	fn handle_mouse_drag(&mut self, pos: vec2f) {
		// crickets...
	}

	fn handle_mouse_up(&mut self, pos: vec2f) {
		const SELECT_RADIUS: f32 = 0.6;
		self.select_crabs(Bounds::around_point(pos, SELECT_RADIUS))
	}

	fn select_crabs(&self, selection: Bounds2f) {
		for crab in self.iter_crablets() {
			let selected = selection.contains(crab.position());
			crab.set_selected(selected);
			if selected {
				self.set_selected_crab(Some(crab.id))
			}
		}
	}

	fn handle_zoom(&mut self) {
		self.camera.zoom = (self.camera.zoom * self.inputs.zoom_delta).clamp(4.0, 256.0);
	}

	fn handle_scroll(&mut self) {
		let sensitivity = 0.03;
		self.pan_camera(self.inputs.scroll_delta * vec2(1.0, -1.0) * sensitivity);
	}

	fn pan_camera(&mut self, delta: vec2f) {
		// zoom-dependent sensitivity so that we pan by same amount of screen pixels
		// (fast zoom when zoomed out, slow when zoomed in)
		self.camera.world_position += delta / self.camera.zoom;

		//self.gamestate.camera.position = self.gamestate.camera.position.map(|v| (v * 32.0).floor() / 32.0);
		// 👈 integer camera position
	}

	// Turn drag start, end into a selection rectangle.
	pub fn selection((start, end): (vec2f, vec2f)) -> Bounds2f {
		Bounds::from_point(start).add(end)
	}
}
