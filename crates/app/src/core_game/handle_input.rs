use crate::*;

impl GameState {
	pub fn tick_inputs(&mut self) {
		self.mouse_pos = self.camera.screen_to_tile(self.inputs.pointer_pos_pix);
		self.handle_zoom();
		self.handle_scroll();
		self.handle_keys();
		self.inputs.reset();
	}

	fn handle_keys(&mut self) {
		let plane = &mut self.plane;
		let throttle_step = plane.max_propeller_force / 100.0;
		let elevator_step = 60.0 * DEG / 100.0;
		for k in &self.inputs.keys_down {
			use egui::Key;
			match k {
				Key::ArrowLeft | Key::S => plane.propeller_force -= throttle_step,
				Key::ArrowRight | Key::F => plane.propeller_force += throttle_step,
				Key::ArrowDown | Key::D => plane.elevator.pitch -= elevator_step,
				Key::ArrowUp | Key::E => plane.elevator.pitch += elevator_step,
				Key::Space => toggle(&mut self.debug.pause_all_systems),
				_ => (),
			};
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
	}
}

fn toggle(v: &mut bool) {
	*v = !*v
}
