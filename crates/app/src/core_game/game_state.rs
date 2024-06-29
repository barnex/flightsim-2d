use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, EguiInspect, Setters)]
#[serde(default)]
pub struct GameState {
	pub frame: u32,
	pub last_frame_micro_timestamp: u64,
	pub last_frame_cpu_micros: u32,
	pub last_frame_micros: u32,
	pub last_fps: f32,
	pub fps_label: String,

	pub camera: Camera,

	#[serde(skip)]
	pub mouse_pos: vec2f,

	pub debug: DebugOpts,

	pub selected_crab: Mut<Option<ID<Crablet>>>,

	pub crablets: CellArena<Crablet>,
	pub plankton: CellArena<Plankton>,
	pub plane: Option<Plane>,

	pub linear_damping: Mut<f32>,
	pub angular_damping: Mut<f32>,

	pub tilemap: Tilemap,

	#[serde(skip)]
	pub inputs: Inputs,

	#[serde(skip)]
	#[inspect(hide)]
	pub stats: Stats,

	#[serde(skip)]
	#[inspect(hide)]
	pub commands: Commands,
}

//pub const MAJOR_TICK_FRAMES: u32 = 16; // TODO

impl GameState {
	pub fn new() -> Self {
		Self { ..default() }
	}
}

//---------------------------------------------------------------- tick
impl GameState {
	pub fn tick(&mut self) {
		self.tick_fps_counter();
		self.stats.start_frame(); // ! Must be first. Zeros current frame stats.

		// inputs processed even when systems paused so we can still move camera etc.
		profiler::scope("tick_inputs", || self.tick_inputs());

		if !self.debug.pause_all_systems {
			self.frame += 1;
		}

		for crab in self.iter_crablets() {
			crab.tick(self)
		}
		for prop in self.plankton.iter() {
			prop.tick(self)
		}
		if let Some(plane) = &self.plane {
			plane.tick(self)
		}

		self.exec_command_queue();
		self.gc();

		self.last_frame_cpu_micros = (micros_since_epoch() - self.last_frame_micro_timestamp).try_into().expect("u32 overflow");
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

	pub fn dt(&self) -> f32 {
		let mut dt = 1.0 / self.last_fps;
		if dt > 0.1 {
			dt = 0.1; // slow down physics if we render < 10 FPS
		}
		if !dt.is_finite() {
			dt = 0.016; // 60 FPS, a good guess
		}
		if dt < 0.02 {
			dt = 0.02;
		}
		dt
	}

	fn gc(&mut self) {}
}

//---------------------------------------------------------------- commands
impl GameState {
	pub fn remove_selected(&mut self) {
		log::warn!("TODO: remove_selected");
		//for crab in self.crablets.iter(){
		//	if crab.selected(){
		//		self.crablets.remove_later(crab.id)
		//	}
		//}
	}

	pub fn manual_tick(&mut self) {
		self.debug.pause_all_systems = false;
		self.tick();
		self.debug.pause_all_systems = true;
	}

	pub fn spawn_crab(&mut self) {
		self.crablets.push_now(Crablet::new(self.camera.world_position));
	}

	pub fn spawn_plankton(&mut self) {
		self.plankton.push_now(Plankton::new(self.camera.world_position));
	}
}

impl GameState {
	pub fn iter_crablets(&self) -> impl Iterator<Item = &Crablet> {
		self.crablets.iter()
	}
}

impl Default for GameState {
	fn default() -> Self {
		let map_size = vec2u(256, 128);
		let mut rng = rand::thread_rng();
		Self {
			frame: 0,
			stats: default(),
			selected_crab: default(),
			mouse_pos: default(),
			inputs: default(),
			camera: default(),
			debug: default(),
			commands: default(),
			last_frame_micro_timestamp: micros_since_epoch(),
			last_frame_micros: 0,
			last_fps: 0.0,
			last_frame_cpu_micros: 0,
			fps_label: default(),
			crablets: CellArena::default().with(|v| {
				v.push_now(Crablet::new((5.0, 3.0).into()));
			}),
			plankton: CellArena::default().with(|v| {
				for _ in 0..1 {
					v.push_now(Plankton::new(20.0 * vec2f(rng.gen(), rng.gen())));
				}
			}),
			plane: None,
			linear_damping: 0.33.into(),
			angular_damping: 0.33.into(),
			tilemap: Tilemap::aquarium(vec2(64, 48)),
		}
	}
}

pub fn micros_since_epoch() -> u64 {
	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("system time").as_micros() as u64
}

/// Util
impl GameState {
	pub fn rng(&self) -> rand::rngs::ThreadRng {
		rand::thread_rng()
	}
}
