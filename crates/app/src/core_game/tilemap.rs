use crate::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, EguiInspect)]
pub struct Tilemap {
	pub tiles: Vec2D<Tile>,
}

impl Tilemap {
	pub fn new(size: vec2u, fill: Tile) -> Self {
		Self { tiles: Vec2D::new(size, fill) }
	}

	pub fn aquarium(size: vec2u) -> Self {
		let (w, h) = (size.as_i32() - 1).into();
		let mut map = Self::new(size, Tile::WATER);

		// sand pile
		let mut rng = rand::thread_rng();
		let mut sand_height = 5;
		for x in 0..w {
			sand_height += rng.gen_range(-1..=1) + rng.gen_range(-1..=1);
			sand_height = sand_height.clamp(2, h / 2);

			for y in 0..=sand_height {
				map.try_set(vec2(x, y), Tile::SAND)
			}
		}

		// borders
		for x in 0..=w {
			map.try_set(vec2(x, 0), Tile::STONE);
			map.try_set(vec2(x, h), Tile::STONE);
		}
		for y in 0..=h {
			map.try_set(vec2(0, y), Tile::STONE);
			map.try_set(vec2(w, y), Tile::STONE);
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
}
