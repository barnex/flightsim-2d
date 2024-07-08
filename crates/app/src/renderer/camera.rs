use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug, EguiInspect)]
pub struct Camera {
	/// screen size in pixels
	pub viewport_size_pix: vec2u,

	/// camera is centered on this game (tile) position
	pub world_position: vec2f,

	/// zoom factor between world (tiles) and screen (pixels)
	pub zoom: f32,

	pub rotation: f32,
	pub pitch: f32,

	/// wgsl frustum z-range. Only points in interval [-z_range..z_range]
	/// (after camera rotation) will be visible.
	/// Should typically be a fairly large range, like 1024.
	pub z_range: f32,
}

impl Camera {
	/// Transform matrix (world to screen)
	pub fn matrix(&self) -> mat4x4f {
		let (dx, dy) = self.world_position.into();
		let delta = vec3(dx, dy, 0.0);
		let viewport = self.viewport_size_pix.as_f32();

		let scale = if viewport.x() > viewport.y() {
			// landscape
			let s = self.zoom / viewport.x();
			let sy = self.zoom / viewport.y();
			2.0 * vec3(s, sy, s / self.z_range)
		} else {
			// portrait
			let s = self.zoom / viewport.y();
			let sx = self.zoom / viewport.x();
			2.0 * vec3(sx, s, s / self.z_range)
		};

		mat4x4::translation(vec3(0.0, 0.0, 0.5)) //_
	 	* mat4x4::scale_anisotropic(scale) //_
	 	* mat4x4::pitch(self.pitch)  //_
	 	* mat4x4::rotation_matrix(vec3::EZ, self.rotation)  //_
	 	* mat4x4::translation(-delta)
	}

	/// convert from screen position (pixels) to game (tile) position
	pub fn screen_to_tile(&self, pos: vec2f) -> vec2f {
		let tile_zoom = self.zoom;
		let x = (pos.x() - self.viewport_size_pix.x() as f32 / 2.0) / tile_zoom + self.world_position.x();
		let y = (-pos.y() + self.viewport_size_pix.y() as f32 / 2.0) / tile_zoom + self.world_position.y();
		vec2f(x, y)
	}

	/// visible part of the world, given current camera position, zoom and screen size.
	pub fn visible_tile_range(&self) -> Bounds2f {
		let offset = vec2(tilemap_x_offset(self.world_position), 0.0);
		let vec([w, h]) = self.viewport_size_pix.as_f32();
		Bounds2f {
			min: self.screen_to_tile(vec2(0.0, h)) - offset,
			max: self.screen_to_tile(vec2(w, 0.0)) + vec::ONES - offset,
		}
	}
}

impl Default for Camera {
	fn default() -> Self {
		Self {
			world_position: default(),
			zoom: 64.0,
			viewport_size_pix: vec::ONES,
			rotation: 0.0,
			pitch: 0.0,
			z_range: 512.0,
		}
	}
}
