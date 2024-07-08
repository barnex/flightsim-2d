use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Tile(pub u8);

#[allow(unused)]
impl Tile {
	pub const AIR: Tile = Tile(0);
	pub const CLOUD: Tile = Tile(1);
	pub const TARMAC: Tile = Tile(2);
	pub const LINE: Tile = Tile(3);
}

impl Tile {
	pub fn can_walk(self) -> bool {
		match self {
			Tile::AIR | Tile::CLOUD => true,
			Tile::TARMAC | Tile::LINE => false,
			_ => false,
		}
	}

	pub fn color(self) -> vec4u8 {
		vec3::from(match self {
			Self::AIR => (141, 154, 255),
			Self::CLOUD => (181, 194, 255),
			Self::TARMAC => (50, 50, 50),
			Self::LINE => (200, 200, 100),
			_ => (255, 0, 0),
		})
		.append(255)
	}

	pub fn height(self) -> f32 {
		match self {
			Tile::TARMAC | Tile::LINE => 1.0,
			Tile::AIR | Tile::CLOUD => -0.5,
			_ => 0.0,
		}
	}
}
