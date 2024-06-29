use crate::*;

impl GameState {
	pub fn draw_on(&self, sg: &mut Scenegraph) {
		// let visible_range = self.camera.visible_tile_range();
		sg.uniforms.camera = self.camera.matrix();
		draw_axes(sg);
		let visible_range = self.camera.visible_tile_range();
		self.draw_tilemap_3d(sg);
		self.draw_props(sg);

		if let Some(plane) = &self.plane{
			plane.draw(sg)
		}

	}

	fn draw_props(&self, sg: &mut Scenegraph) {
		sg.new_layer(); // 👈  draw atop tiles

		const MAX_SPRITE_SIZE: f32 = 4.0; // tiles

		let visible_range = self //.
			.camera
			.visible_tile_range()
			.with_margin(MAX_SPRITE_SIZE);

		for crab in self.crablets.iter().filter(|crab| visible_range.contains(crab.position())) {
			const SELECT_COLOR: vec4f = vec4f(0.5, 0.5, 1.0, 0.6);
			sg.push(QuadInstanceData::new(crab.position(), crab.sprite()).with(|d| {
				d.mix_color = select(SELECT_COLOR, vec4f::ZERO, crab.selected());
				d.scale = crab.scale();
				d.rotation = crab.rotation();
			}));
		}

		for prop in self.plankton.iter().filter(|crab| visible_range.contains(crab.position())) {
			const SELECT_COLOR: vec4f = vec4f(0.5, 0.5, 1.0, 0.6);
			sg.push(QuadInstanceData::new(prop.position(), prop.sprite()).with(|d| {
				d.mix_color = select(SELECT_COLOR, vec4f::ZERO, prop.selected());
				d.scale = prop.scale();
				d.rotation = prop.rotation();
			}));
		}
	}

	pub fn draw_tilemap_3d(&self, sg: &mut Scenegraph) {
		sg.uniforms.camera = self.camera.matrix();
		draw_axes(sg);

		for tile_p in self.visible_tile_range().iter_excl() {
			// TODO: can avoid bound checks when carefully intersecting visible range with tile range
			let tile = self.tilemap._at(tile_p.as_u32());

			let height = tile.height();

			let center = (tile_p.as_f32()).append(height);

			let corners = [(0, 0), (1, 0), (1, 1), (0, 1)].map(vec2::from);

			let mut quad: [TerrainVertex; 4] = default();
			for (i, corner) in corners.into_iter().enumerate() {
				let mut sum_light = 0.0f32;
				for neighbor in (-1..=0).cartesian_product(-1..=0).map(|(dx, dy)| tile_p + vec2(dx, dy) + corner) {
					if self.tilemap.at_pos(neighbor).height() <= height {
						sum_light += 0.25; // TODO: correclty occlude diagonal
					}
				}

				let base_color = tile.color().map(|v| (v as f32) / 255.0);
				let lit_color = base_color * sum_light;
				let color_rgba = (lit_color * 255.0).as_u8();

				let vertex = TerrainVertex {
					position: center + corner.append(1).as_f32(),
					color: pack4xu8(color_rgba),
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
