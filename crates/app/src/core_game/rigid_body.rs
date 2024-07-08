use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect)]
pub struct RigidBody {
	pub mass: f32,
	pub position: vec2f,
	pub velocity: vec2f,
	pub acceleration: vec2f,

	pub rot_inertia: f32,
	pub rotation: f32,
	pub rot_velocity: f32,
	pub rot_accel: f32,
}

impl Default for RigidBody {
	fn default() -> Self {
		Self {
			mass: 1.0,
			rot_inertia: 1.0,
			position: default(),
			velocity: default(),
			acceleration: default(),
			rotation: default(),
			rot_velocity: default(),
			rot_accel: default(),
		}
	}
}

impl RigidBody {
	pub fn update_accel(&mut self, force: vec2f, torque: f32) {
		self.acceleration = force / self.mass;
		self.rot_accel = torque / self.rot_inertia;
	}

	pub fn update_velocity(&mut self, dt: f32) {
		self.velocity += dt * self.acceleration;
		self.rot_velocity += dt * self.rot_accel;
	}

	pub fn update_position(&mut self, dt: f32) {
		self.position += dt * self.velocity;
		self.rotation += dt * self.rot_velocity;
		if self.rotation > PI {
			self.rotation -= 2.0 * PI;
		} else if self.rotation < -PI {
			self.rotation += 2.0 * PI;
		}
	}

	pub fn tick_with_tilemap(&mut self, dt: f32, tilemap: &Tilemap, force: vec2f, torque: f32) {
		// translation
		self.acceleration = force / self.mass;
		self.velocity += dt * force / self.mass;

		//self.set_velocity((1.0 - dt * gs.linear_damping()) * self.velocity);
		let delta_pos = dt * self.velocity;
		let new_pos = self.position + delta_pos;
		if can_walk(new_pos) { // 👈 hack for infite sized tile map
			// no collision
			self.position = new_pos;
		} else {
			// collision: bounce along normal direction, keep moving along tangent:
			for i in 0..2 {
				// x, y
				let new_pos = self.position + delta_pos.with(|v| v[1 - i] = 0.0);
				if can_walk(new_pos) {
					self.position = new_pos;
				} else {
					self.velocity = self.velocity.with(|v| v[i] = -0.5 * v[i]);
				}
			}
		}

		// rotation
		let mut theta = self.rotation;
		theta += dt * self.rot_velocity;
		if theta > PI {
			theta -= 2.0 * PI;
		} else if theta < -PI {
			theta += 2.0 * PI;
		}
		self.rotation = theta;

		const MAX_ROT: f32 = 10.0;
		self.rot_accel = torque / self.rot_inertia;
		self.rot_velocity += dt * self.rot_accel;
	}

	pub fn inverse_rotation_matrix(&self) -> mat2x2f {
		let (sin, cos) = f32::sin_cos(self.rotation);
		mat2x2f::from([[cos, sin], [-sin, cos]])
	}

	pub fn rotation_matrix(&self) -> mat2x2f {
		let (sin, cos) = f32::sin_cos(self.rotation);
		mat2x2f::from([[cos, -sin], [sin, cos]])
	}

	pub fn transform_rel_pos(&self, rel_pos: vec2f) -> vec2f {
		(self.rotation_matrix() * rel_pos) + self.position
	}

	pub fn transform_vector(&self, vector: vec2f) -> vec2f {
		self.rotation_matrix() * vector
	}

	pub fn velocity_of_rel_pos(&self, rel_pos: vec2f) -> vec2f {
		let (x, y) = rel_pos.into();
		let rot_vel = 2.0 * PI * self.rot_velocity * vec2(-y, x);
		rot_vel + self.velocity
	}

	pub fn transform_rotation(&self, rotation: f32) -> f32 {
		self.rotation + rotation
	}

	pub fn transform_frame(&self, (pos, rot): (vec2f, f32)) -> (vec2f, f32) {
		(self.transform_rel_pos(pos), self.transform_rotation(rot))
	}
}
