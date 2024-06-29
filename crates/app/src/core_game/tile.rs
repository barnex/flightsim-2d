use crate::prelude::*;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Tile(pub u8);

#[allow(unused)]
impl Tile {
	pub const WATER: Tile = Tile(0);
	pub const SAND: Tile = Tile(1);
	pub const FARM_LAND: Tile = Tile(4);
	pub const STONE: Tile = Tile(2);
	pub const LAVA: Tile = Tile(3);
	pub const ROAD: Tile = Tile(6);
}

impl Tile {
	pub fn sprite(self) -> Sprite {
		const OFFSET: u8 = 8; // Tile 0 is sprite 8 in the atlas
		Sprite(self.0 + OFFSET)
	}

	pub fn can_walk(self) -> bool {
		match self{
			Tile::WATER => true,
			Tile::SAND => false,
			_ => false,
		}
	}

	pub fn is_road(self) -> bool {
		self == Tile::ROAD
	}

	pub fn color(self) -> vec4u8 {
		vec3::from(match self {
			Self::WATER => (101, 114, 255),
			Self::FARM_LAND => (29, 174, 85),
			Self::STONE => (121, 121, 121),
			Self::LAVA => (220, 37, 0),
			Self::SAND => (249, 240, 143),
			Self::ROAD => (69, 80, 81),
			_ => (255, 0, 0),
		})
		.append(255)
	}

	pub fn height(self) -> f32 {
		match self {
			Tile::STONE => 1.0,
			Tile::SAND => -0.2,
			Tile::ROAD => -0.1,
			Tile::LAVA => -0.3,
			Tile::WATER => -0.5,
			_ => 0.0,
		}
	}
}
