use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect, Setters)]
pub struct Plane {
	pub id: ID<Self>,
	pub physics: Physics,
}

impl Plane {
	pub fn new(position: vec2f) -> Self {
		Self {
			id: default(),

			physics: Physics {
				position: position.into(),
				..default()
			},
		}
	}

	pub fn tick(&self, gs: &GameState) {
		self.tick_physics(gs);
	}

	pub fn tick_physics(&self, gs: &GameState) {
		// let [power_left, power_right] = self.flipper_power();
		// let force = vec(self.physics.rotation_matrix()[1]) * (power_left + power_right);
		// let torque = power_left - power_right; // <<<<<< TODO: * arm length
		self.physics.tick(gs, vec::ZERO, 0.0);
	}

	pub fn position(&self) -> vec2f {
		self.physics.position()
	}

	pub fn rotation(&self) -> f32 {
		self.physics.rotation()
	}

	pub fn draw(&self, sg: &mut Scenegraph) {
		sg.push(QuadInstanceData::new(self.position(), Sprite::PLANE).with(|d| {
			d.scale = 10.0;
			d.rotation = self.rotation();
		}));
		sg.push(QuadInstanceData::new(self.position(), Sprite::WING).with(|d| {
			d.scale = 3.0;
			d.rotation = self.rotation();
			d.position[2] = 0.1;
		}));
	}
}

impl HasID for Plane {
	fn id_mut(&mut self) -> &mut ID<Self> {
		&mut self.id
	}
}

pub fn airplaine_gamestate() -> GameState {
	GameState {
		plane: Some(Plane::new(vec2(10.0, 10.0))),
		..default()
	}
}
