use crate::*;

// TileMap repeats after this number of tiles.
pub const TILEMAP_WRAP: i32 = 512;

impl GameState {
	pub fn draw_on(&self, sg: &mut Scenegraph) {
		sg.clear_color = vec4(0.6, 0.7, 1.0, 1.0); // sky blue

		sg.uniforms.camera = self.camera.matrix();
		if self.debug.draw_axes {
			draw_axes(sg);
		}

		self.draw_tilemap_3d(sg);
		self.plane.draw(sg)
	}

	pub fn draw_tilemap_3d(&self, sg: &mut Scenegraph) {
		if !self.debug.draw_tilemap {
			return;
		}

		sg.uniforms.camera = self.camera.matrix();

		if !self.plane.body.position.all(|v| v.is_finite()) {
			return;
		}

		let offset = tilemap_x_offset(self.plane.body.position);

		'tiles: for tile_p in self.visible_tile_range().iter_excl() {
			let tile = self.tilemap._at(tile_p.as_u32());

			if tile == Tile::AIR {
				continue 'tiles; // 👈 no need to draw air
			}

			let height = tile.height();

			let center = (tile_p.as_f32()).with(|v| v[0] += offset).append(height);

			let corners = [(0, 0), (1, 0), (1, 1), (0, 1)].map(vec2::from);

			let mut quad: [TerrainVertex; 4] = default();
			for (i, corner) in corners.into_iter().enumerate() {
				let vertex = TerrainVertex {
					position: center + corner.append(1).as_f32(),
					color: pack4xu8(tile.color()),
				};
				quad[i] = vertex;
			}

			sg.meshbuffer.push_rect(&quad);
		}
	}

	pub fn visible_tile_range(&self) -> Bounds2i {
		self //.
			.camera
			.visible_tile_range()
			.map(|b| b.floor())
			.intersect(self.tilemap.bounds().as_i32())
	}
}

fn draw_axes(sg: &mut Scenegraph) {
	let origin = unit_cube();

	sg.meshbuffer.extend(origin.vertices.iter().cloned(), &origin.indices);
	sg.meshbuffer.extend(
		origin.vertices.iter().map(|v| TerrainVertex {
			position: v.position + vec3::EX * 2.0,
			color: pack4xu8((255, 0, 0, 255)),
		}),
		&origin.indices,
	);
	sg.meshbuffer.extend(
		origin.vertices.iter().map(|v| TerrainVertex {
			position: v.position + vec3::EY * 2.0,
			color: pack4xu8((0, 255, 0, 255)),
		}),
		&origin.indices,
	);
	sg.meshbuffer.extend(
		origin.vertices.iter().map(|v| TerrainVertex {
			position: v.position + vec3::EZ * 2.0,
			color: pack4xu8((0, 0, 255, 255)),
		}),
		&origin.indices,
	);
}

pub fn tilemap_x_offset(plane_pos: vec2f) -> f32 {
	(((plane_pos.x().floor() as i32) / TILEMAP_WRAP) * TILEMAP_WRAP - TILEMAP_WRAP / 2) as f32
}
