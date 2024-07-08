use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, EguiInspect)]
#[serde(default)]
pub struct GameState {
	pub frame: u32,
	pub last_frame_micro_timestamp: u64,
	pub last_frame_cpu_micros: u32,
	pub last_frame_micros: u32,
	pub last_fps: f32,
	pub fps_label: String,

	pub camera: Camera,
	pub camera_follows: bool,

	// exponential smoothing coefficient 0..1
	pub camera_follow_speed: f32,

	// buffer camera postition for exponential smoothing
	#[inspect(hide)]
	pub camera_follow_buf: vec2f,

	#[serde(skip)]
	pub mouse_pos: vec2f,

	pub debug: DebugOpts,

	pub plane: Plane,

	#[serde(skip)]
	pub tilemap: Tilemap,

	#[serde(skip)]
	pub inputs: Inputs,

	#[inspect(hide)]
	pub plotter: Plotter,
}

impl GameState {
	pub fn tick(&mut self) {
		self.tick_fps_counter();

		// inputs processed even when systems paused so we can still move camera etc.
		self.tick_inputs();

		if self.debug.pause_all_systems {
			self.plane.tick(0.0, &self.tilemap);
			if self.debug.force_record_plots {
				self.record_plot()
			}
		} else {
			// TODO: this assumes 60 FPS
			for _ in 0..(16 * self.debug.timepassage) {
				self.inner_tick()
			}
			self.record_plot()
		}

		self.tick_camera();

		if self.plane.position().y() < 0.0 {
			self.debug.pause_all_systems = true; // stop simulation on crash
		}

		self.last_frame_cpu_micros = (micros_since_epoch() - self.last_frame_micro_timestamp).try_into().expect("u32 overflow");
	}

	pub fn inner_tick(&mut self) {
		// fixed 1ms physics timestep
		self.plane.tick(0.001 /*dt*/, &self.tilemap);
		self.frame += 1;
		self.record_plot();
	}

	pub fn record_plot(&mut self) {
		let t = self.frame as f32 / 1000.0;
		let body = &self.plane.body;
		self.plotter.pushf(|| {
			vec![
				t, //_
				body.position.x(),
				body.position.y(),
				body.velocity.x(),
				body.velocity.y(),
				body.acceleration.x(),
				body.acceleration.y(),
				body.rotation / DEG,
				body.rot_velocity / DEG,
				body.rot_accel / DEG,
				self.plane.wings_aoa() / DEG,
				self.plane.winglet_lift(&self.plane.wings).len(),
				self.plane.winglet_induced_drag(&self.plane.wings).len(),
				(body.acceleration + vec2f(0.0, self.plane.gravity)).len() / self.plane.gravity,
			]
		});
	}

	fn tick_fps_counter(&mut self) {
		let now = micros_since_epoch() as i64;
		let last = self.last_frame_micro_timestamp as i64;

		self.last_frame_micros = (now - last).try_into().unwrap_or(0);

		if self.last_frame_micros < 1 {
			self.last_frame_micros = 1;
		}
		if self.last_frame_micros > 1000 {
			self.last_frame_micros = 1000;
		}

		let fps = 1.0 / ((self.last_frame_micros as f32) / 1e6);
		self.last_fps = 0.95 * self.last_fps + 0.05 * fps;

		if !self.last_fps.is_finite() {
			self.last_fps = 0.0;
		}

		use std::fmt::Write;
		self.fps_label.clear();
		write!(&mut self.fps_label, "{:.1} ms | {:3.0} fps", (self.last_frame_cpu_micros as f32) / 1000.0, self.last_fps).expect("format");

		self.last_frame_micro_timestamp = now as u64;
	}

	fn tick_camera(&mut self) {
		let a = self.camera_follow_speed.clamp(0.0, 1.0);
		let b = 1.0 - a;
		self.camera_follow_buf = a * self.plane.position() + b * self.camera_follow_buf;
		if self.camera_follows {
			self.camera.world_position = a * self.camera_follow_buf + b * self.camera.world_position;
		}

		if !self.camera.world_position.iter().all(|v| v.is_finite()) {
			self.camera.world_position = default();
			self.camera_follow_buf = default();
		}
	}
}

impl Default for GameState {
	fn default() -> Self {
		Self {
			frame: 0,
			mouse_pos: default(),
			inputs: default(),
			camera: default(),
			camera_follows: true,
			camera_follow_speed: 0.4,
			camera_follow_buf: default(),
			debug: default(),
			last_frame_micro_timestamp: micros_since_epoch(),
			last_frame_micros: 0,
			last_fps: 0.0,
			last_frame_cpu_micros: 0,
			fps_label: default(),
			plane: Plane::default(),
			tilemap: Tilemap::airstrip(vec2(1024, 1024)),
			plotter: Plotter::new(&[
				"t (s)", //_
				"x position (m)",
				"y position (m)",
				"x velocity (m/s)",
				"y velocity (m/s)",
				"x acceleration (m/s²)",
				"y acceleration (m/s²)",
				"pitch (deg)",
				"rot. vel (deg/s)",
				"torque (deg/s²)",
				"aoa (deg)",
				"lift (N)",
				"drag (N)",
				"G force",
			]),
		}
	}
}

pub fn micros_since_epoch() -> u64 {
	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("system time").as_micros() as u64
}
