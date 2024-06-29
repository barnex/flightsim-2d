use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect, Setters)]
pub struct Plankton {
	pub id: ID<Self>,
	pub sprite: Mut<Sprite>,
	pub scale: Mut<f32>,
	pub selected: Mut<bool>,
	pub physics: Physics,
}

impl Plankton {
	pub fn new(position: vec2f) -> Self {
		Self {
			physics: Physics {
				position: position.into(),
				..default()
			},
			..default()
		}
	}

	pub fn tick(&self, gs: &GameState) {
		if !gs.debug.tick_plankton{
			return
		}

		let mut rng = rand::thread_rng();
		let force = 0.5 * (vec2f(rng.gen(), rng.gen()) - 0.5);
		let torque = 0.5 * (rng.gen::<f32>() - 0.5);
		self.physics.tick(gs, force, torque);
	}

	pub fn position(&self) -> vec2f {
		self.physics.position()
	}
	pub fn rotation(&self) -> f32 {
		self.physics.rotation()
	}
}

impl Default for Plankton {
	fn default() -> Self {
		Self {
			sprite: Sprite::LEAF.into(),
			scale: 0.35.into(),
			id: default(),
			selected: default(),
			physics: default(),
		}
	}
}

impl HasID for Plankton {
	fn id_mut(&mut self) -> &mut ID<Self> {
		&mut self.id
	}
}
