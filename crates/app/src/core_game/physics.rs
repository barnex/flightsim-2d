use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect, Setters, Default)]
pub struct Physics {
	pub position: Mut<vec2f>,
	pub velocity: Mut<vec2f>,
	pub rotation: Mut<f32>,
	pub angular_vel: Mut<f32>,
}

impl Physics {
	pub fn tick(&self, gs: &GameState, force: vec2f, torque: f32) {
		let dt = gs.dt();

		// translation
		self.set_velocity(self.velocity() + dt * force);
		self.set_velocity((1.0 - dt * gs.linear_damping()) * self.velocity());
		let delta_pos = dt * self.velocity();
		let new_pos = self.position() + delta_pos;
		if gs.tilemap.at_pos(new_pos.floor()).can_walk() {
			// no collision
			self.set_position(new_pos);
		} else {
			// collision: bounce along normal direction, keep moving along tangent:
			for i in 0..2 { // x, y
				let new_pos = self.position() + delta_pos.with(|v| v[1-i] = 0.0);
				if gs.tilemap.at_pos(new_pos.floor()).can_walk() {
					self.set_position(new_pos);
				} else {
					self.set_velocity(self.velocity().with(|v| v[i] = -0.5 * v[i]));
				}
			}
		}

		// rotation
		let mut theta = self.rotation();
		theta += dt * self.angular_vel();
		if theta > PI {
			theta -= 2.0 * PI;
		} else if theta < -PI {
			theta += 2.0 * PI;
		}
		self.set_rotation(theta);

		self.set_angular_vel(self.angular_vel() + dt * torque);
		self.set_angular_vel((1.0 - dt * gs.angular_damping()) * self.angular_vel())
	}

	pub fn rotation_matrix(&self) -> mat2x2f {
		let (sin, cos) = f32::sin_cos(self.rotation());
		mat2x2f::from([[cos, sin], [-sin, cos]])
	}

	pub fn inverse_rotation_matrix(&self) -> mat2x2f {
		let (sin, cos) = f32::sin_cos(-self.rotation());
		mat2x2f::from([[cos, sin], [-sin, cos]])
	}
}
