use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect, Setters)]
pub struct Crablet {
	pub id: ID<Self>,
	pub sprite: Mut<Sprite>,
	pub scale: Mut<f32>,
	pub selected: Mut<bool>,
	pub name: String,

	pub physics: Physics,

	pub retina: Mut<vec2f>,
	pub brain: RefCell<Brain1>,

	pub manual_control: bool,

	// actuation power of left and right flippers. -1.0 (full backwards) .. 1.0 (full forwards)
	pub flipper_power: Mut<[f32; 2]>,
}

impl Crablet {
	pub fn new(position: vec2f) -> Self {
		Self {
			id: default(),
			sprite: Sprite::FERRIS_UNSAFE.into(),
			scale: 1.0.into(),
			selected: false.into(),
			name: default(),

			physics: Physics {
				position: position.into(),
				..default()
			},
			retina: default(),
			brain: RefCell::new(Brain1 {
				weights: default(),
				biases: default(),
			}),
			manual_control: default(),
			flipper_power: default(),
		}
	}

	pub fn tick(&self, gs: &GameState) {
		self.tick_retina(gs);
		self.tick_brain(gs);
		self.tick_physics(gs);
	}

	pub fn tick_retina(&self, gs: &GameState) {
		let mut retina = vec::ZERO;

		for prop in gs.plankton().iter() {
			let rel_pos = prop.position() - self.position();
			let dist = rel_pos.len();
			let rel_pos = self.physics.rotation_matrix() * rel_pos; // <<<<<<<<<<<< wrong signs!!!!!!!!!
			if rel_pos.y() > 0.0 {
				let cos_theta = rel_pos.normalized().y();
				if rel_pos.x() < 0.0 {
					retina[0] += cos_theta * (1.0 / dist).clamp(0.0, 1.0);
				} else {
					retina[1] += cos_theta * (1.0 / dist).clamp(0.0, 1.0);
				}
			}
		}

		self.set_retina(retina);
	}

	// TODO: obviously, interferes with flipper controls
	pub fn tick_brain(&self, gs: &GameState) {
		if self.manual_control {
			return;
		}

		// TODO: brain tick is immutable, takes &input (retina) and &output (flippers)
		let mut flipper_power = self.flipper_power().into();
		self.brain.borrow().tick(&mut flipper_power, &self.retina());
		self.set_flipper_power(flipper_power.into());
	}

	pub fn tick_physics(&self, gs: &GameState) {
		let [power_left, power_right] = self.flipper_power();
		let force = vec(self.physics.rotation_matrix()[1]) * (power_left + power_right);
		let torque = power_left - power_right; // <<<<<< TODO: * arm length
		self.physics.tick(gs, force, torque);
	}

	pub fn position(&self) -> vec2f {
		self.physics.position()
	}
	pub fn rotation(&self) -> f32 {
		self.physics.rotation()
	}
}

impl HasID for Crablet {
	fn id_mut(&mut self) -> &mut ID<Self> {
		&mut self.id
	}
}
