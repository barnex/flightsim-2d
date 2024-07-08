use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, EguiInspect)]
pub struct Tilemap {
	pub tiles: Vec2D<Tile>,
}

impl Tilemap {
	pub fn new(size: vec2u, fill: Tile) -> Self {
		Self { tiles: Vec2D::new(size, fill) }
	}

	pub fn airstrip(size: vec2u) -> Self {
		let (w, h) = (size.as_i32() - 1).into();
		let mut map = Self::new(size, Tile::AIR);

		for x in (0..w).step_by(32) {
			for y in (0..h).step_by(32) {
				map.try_set((x, y).into(), Tile::CLOUD);
			}
		}

		// borders
		let h = 4;
		for x in 0..=w {
			for y in 0..=h {
				map.try_set(vec2(x, y), Tile::TARMAC);
			}

			if (x / 8) % 2 == 0 {
				map.try_set(vec2(x, h), Tile::LINE);
			}
		}

		for y in 0..=h {
			map.try_set(vec2(0, y), Tile::TARMAC);
		}

		map
	}

	pub fn _at(&self, index: vec2u) -> Tile {
		*self.tiles._at(index)
	}

	pub fn __try_at(&self, index: Pos) -> Option<Tile> {
		self.tiles.try_at(index).copied()
	}

	pub fn at_pos(&self, index: Pos) -> Tile {
		self.tiles.try_at(index).copied().unwrap_or_default()
	}

	pub fn at_mut(&mut self, index: vec2u) -> &mut Tile {
		self.tiles.at_mut(index)
	}

	pub fn try_set(&mut self, index: Pos, value: Tile) {
		self.tiles.try_set(index, value)
	}

	pub fn bounds(&self) -> Bounds2u {
		self.tiles.bounds()
	}

	pub fn iter_range_excl(&self, bounds: Bounds2i) -> impl Iterator<Item = (Pos, Tile)> + '_ {
		self.tiles.iter_range_excl(bounds)
	}

	pub fn iter(&self) -> impl Iterator<Item = (vec2u, Tile)> + '_ {
		self.tiles.iter().map(|(pos, tile)| (pos, *tile))
	}

	pub fn _can_walk(&self, pos: vec2f) -> bool {
		self.at_pos(pos.floor()).can_walk()
	}
}

pub fn can_walk(pos: vec2f) -> bool {
	pos.y() > 5.0
}
