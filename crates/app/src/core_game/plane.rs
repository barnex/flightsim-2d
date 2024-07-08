use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect)]
pub struct Plane {
	pub body: RigidBody,

	//	pub display_position: vec2f,
	pub gravity: f32,

	pub wings: Winglet,
	pub elevator: Winglet,
	pub wheels: [vec2f; 2],

	pub body_drag: f32,

	pub propeller_force: f32,
	pub max_propeller_force: f32,
	pub forces: RefCell<Vec<Force>>,
	pub draw_forces: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect)]
pub struct Force {
	pub rel_pos: vec2f,
	pub vector: vec2f,
}

impl Plane {
	pub fn default() -> Self {
		Self {
			//display_position: default(),
			body_drag: 0.2,
			gravity: 9.81,
			draw_forces: true,

			wings: Winglet {
				pos: vec2(0.2, 0.5),
				pitch: 3.0 * DEG,
				drag_factor: 1.0,
				lift_to_drag: 15.0,
			},
			elevator: Winglet {
				pos: vec2(-3.7, 0.3),
				pitch: 0.0,
				drag_factor: 0.2,
				lift_to_drag: 15.0,
			},
			wheels: [vec2(-2.5, -0.75), vec2(1.0, -1.40)],
			propeller_force: 0.0,
			max_propeller_force: 2000.0,

			body: RigidBody {
				position: vec2(8.0, 6.5),
				mass: 1000.0,
				rot_inertia: 2000.0,
				..default()
			},

			forces: default(),
		}
	}

	const CENTER_OF_MASS: vec2f = vec2::ZERO;

	pub fn weight(&self) -> f32 {
		self.body.mass * self.gravity
	}

	pub fn base_speed(&self) -> f32 {
		// drag  == max_force
		// v^2 * base_drag == max_force
		// v == sqrt(max_force / base_drag)
		(self.max_propeller_force / self.base_drag()).sqrt()
	}

	pub fn base_drag(&self) -> f32 {
		self.body_drag + self.wings.drag_factor + self.elevator.drag_factor
	}

	pub fn wings_aoa(&self) -> f32 {
		self.winglet_aoa(&self.wings)
	}

	pub fn elevator_aoa(&self) -> f32 {
		self.winglet_aoa(&self.elevator)
	}

	pub fn tick(&mut self, dt: f32, tilemap: &Tilemap) {
		self.update_forces(tilemap);
		self.body.update_position(dt / 2.0);
		self.body.update_velocity(dt);
		self.body.update_position(dt / 2.0);

		self.body.rot_velocity *= 0.999; // some damping
	}

	pub fn update_forces(&mut self, tilemap: &Tilemap) {
		let forces = &mut self.forces.borrow_mut();
		forces.clear();

		// weight
		forces.push(Force {
			rel_pos: Self::CENTER_OF_MASS,
			vector: (-self.gravity * self.body.mass) * vec2::EY,
		});

		// propeller
		forces.push(Force {
			rel_pos: vec2(2.0, 0.0),
			vector: self.propeller_force * (self.body.rotation_matrix() * vec2::EX),
		});

		//wheels
		for &rel_pos in &self.wheels {
			let abs_pos = self.body.transform_rel_pos(rel_pos);
			if !can_walk(abs_pos) {
				let mut probe = abs_pos;
				let mut depth = 0.0;
				let step = 0.01;
				while !can_walk(probe) && depth < 2.0 {
					probe[1] += step;
					depth += step;
				}

				let v_wheel = self.body.velocity_of_rel_pos(rel_pos);
				let spring_k = 30.0;
				let damping = vec2(0.05, 100.0); // wheel damping: small wheel friction (x) + heavy vertical damping (y)
				let force = vec2(0.0, spring_k * self.body.mass * depth) - v_wheel * damping;

				forces.push(Force { rel_pos, vector: force });
			}
		}

		// wings
		for winglet in [self.wings, self.elevator] {
			forces.push(Force {
				rel_pos: winglet.pos,
				vector: self.winglet_force(&winglet),
			})
		}

		// body drag
		forces.push(Force {
			rel_pos: Self::CENTER_OF_MASS,
			vector: self.winglet_force(&Winglet {
				pos: vec::ZERO,
				pitch: 0.0,
				drag_factor: self.body_drag,
				lift_to_drag: 0.0,
			}),
		});

		let (total_force, total_torque) = Self::add_forces(forces);
		self.body.update_accel(total_force, total_torque);
	}

	pub fn winglet_force(&self, winglet: &Winglet) -> vec2f {
		self.winglet_induced_drag(winglet) + self.winglet_lift(winglet)
	}

	pub fn winglet_lift(&self, winglet: &Winglet) -> vec2f {
		let v = self.body.velocity.normalized();
		let aoa = self.winglet_aoa(winglet);

		if aoa.abs() > 90.0 * DEG {
			return vec::ZERO;
		}

		let general_direction = mat2x2::rotation(aoa) * v;
		let rota = rot90(self.body.velocity.normalized());
		let rotb = -rot90(self.body.velocity.normalized());

		let lift_dir = if general_direction.dot(rota) > general_direction.dot(rotb) { rota } else { rotb };

		//let lift_dir = if aoa > 0.0 { up } else { -up };

		lift_dir //_
		* winglet.lift_to_drag //_
		* Self::flow_factor(aoa)//_
		* self.winglet_induced_drag(winglet).len()
	}

	fn flow_factor(aoa: f32) -> f32 {
		f32::cos(aoa).max(0.0)
	}

	pub fn winglet_induced_drag(&self, winglet: &Winglet) -> vec2f {
		winglet.drag_factor //_
		* self.winglet_aoa(winglet).sin().abs() //_
		* self.body.velocity.len2() //_
		* (-self.body.velocity.normalized()) //_
	}

	fn drag(&self, drag_tensor: vec2f, tensor_rotation: f32) -> vec2f {
		let v_relative = mat2x2::rotation(-tensor_rotation) * self.body.velocity;
		let magnitude = drag_tensor.dot(v_relative * v_relative);
		magnitude * (-self.body.velocity.normalized())
	}

	pub fn winglet_aoa(&self, winglet: &Winglet) -> f32 {
		let aoa = if self.body.velocity.len2() < 1.0 {
			0.0 // no noise when velocity is ~zero
		} else {
			self.body_aoa() + winglet.pitch
		};

		check(aoa);
		aoa
	}

	pub fn body_aoa(&self) -> f32 {
		//let speed = self.body.velocity.normalized();
		//let direction = self.body.transform_vector(vec2(1.0, 0.0));
		//speed.dot(direction).acos() * speed.cross(direction).signum()

		let v = self.body.velocity;
		let theta_speed = f32::atan2(v.y(), v.x());
		let direction = self.body.rotation;
		direction - theta_speed
	}

	pub fn body_drag(&self) -> vec2f {
		self.drag(vec2(0.0, self.body_drag), self.body.rotation)
	}

	fn add_forces(forces: &[Force]) -> (vec2f, f32) {
		let mut total_force = vec2::ZERO;
		let mut total_torque = 0.0;
		for force in forces.iter() {
			total_force += force.vector;
			total_torque += force.rel_pos.cross(force.vector);
		}
		(total_force, total_torque)
	}

	pub fn position(&self) -> vec2f {
		self.body.position
	}

	pub fn pitch(&self) -> f32 {
		self.body.rotation
	}

	pub fn draw(&self, sg: &mut Scenegraph) {
		// fuselage
		sg.push(QuadInstanceData::new(self.position(), Sprite::PLANE).with(|d| {
			d.scale = vec2(8.0, 4.0);
			d.rotation = self.pitch();
		}));

		// wings
		{
			let (pos, rot) = self.body.transform_frame((self.wings.pos, self.wings.pitch));
			sg.push(QuadInstanceData::new(pos, Sprite::WING).with(|d| {
				d.scale = vec2(2.5, 1.25);
				d.rotation = rot;
				d.position[2] = 0.1;
			}));
		}

		// elevator
		{
			let (pos, rot) = self.body.transform_frame((self.elevator.pos, self.elevator.pitch));
			sg.push(QuadInstanceData::new(pos, Sprite::WING).with(|d| {
				d.scale = vec2(1.75, 0.75);
				d.rotation = rot;
				d.position[2] = 0.1;
			}));
		}

		//wheels
		for i in 0..2 {
			let (pos, rot) = self.body.transform_frame((self.wheels[i], 0.0));
			sg.push(QuadInstanceData::new(pos, Sprite::WHEEL).with(|d| {
				d.scale = vec2(0.7, 0.7);
				d.rotation = rot;
				d.position[2] = 0.1;
			}));
		}

		// center of mas
		let (pos, rot) = self.body.transform_frame((vec2(0.0, 0.0), 0.0));
		sg.push(QuadInstanceData::new(pos, Sprite::CENTER).with(|d| {
			d.scale = vec2(0.5, 0.5);
			d.rotation = rot;
			d.position[2] = 0.1;
		}));

		if self.draw_forces {
			self.draw_forces(sg);
			self.draw_relative_arrow(sg, vec2::ZERO, self.body.velocity / 10.0, BLUE)
		}
	}

	pub fn draw_forces(&self, sg: &mut Scenegraph) {
		const METER_PER_NEWTON: f32 = 1.0 / 500.0;
		for force in self.forces.borrow().iter() {
			self.draw_relative_arrow(sg, force.rel_pos, force.vector * METER_PER_NEWTON, RED);
		}
	}

	pub fn draw_relative_arrow(&self, sg: &mut Scenegraph, rel_pos: vec2f, vector: vec2f, color: vec4u8) {
		if vector.len2() < 0.1 {
			return;
		}
		let vec([x, y]) = vector.normalized();
		let matrix = mat([[x, -y], [y, x]]);
		let len = vector.len();
		let vertices = [vec2(0.0, -0.3), vec2(0.0, 0.3), vec2(len, 0.0)];
		let indices = [0, 1, 2];
		sg.meshbuffer.extend(
			vertices.map(|v| TerrainVertex::new(((matrix * v) + self.body.transform_rel_pos(rel_pos)).append(2.0)).with(|v| v.color = pack4xu8(color))),
			indices.iter(),
		);
	}
}

fn rot90(v: vec2f) -> vec2f {
	vec2(-v.y(), v.x())
}

#[track_caller]
fn check(v: f32) {
	if !v.is_finite() {
		panic!("lambda the infinite: {v}");
	}
}
