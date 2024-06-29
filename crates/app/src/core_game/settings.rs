use crate::prelude::*;

/// User settings, as defined by "settings.toml".
#[derive(Clone)]
pub struct Settings {
	pub graphics: GraphicsOpts,
	pub controls: Controls,
	pub player: PlayerOpts,
	pub sound: SoundOpts,
	pub network: NetworkOpts,
	pub debug: DebugOpts,
}

impl Settings {
	pub fn todo_load_settings() -> Self {
		Self {
			graphics: default(),
			controls: default(),
			player: default(),
			sound: default(),
			network: default(),
			debug: default(),
		}
	}
}

/// All user-controlled settings, read from "settings.toml".
#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct SettingsToml {
	pub graphics: GraphicsOpts,
	pub controls: Controls,
	pub player: PlayerOpts,
	pub sound: SoundOpts,
	pub network: NetworkOpts,
	#[serde(default)]
	pub debug: DebugOpts,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Controls {
	pub forward: char,
	pub left: char,
	pub backward: char,
	pub right: char,
	pub crouch: char,
	pub mouse_sensitivity: f32,
	pub mouse_stutter_filter: u8,
	pub mouse_smoothing: f32,
}

impl Default for Controls {
	fn default() -> Self {
		Self {
			forward: 'w',
			left: 'a',
			backward: 's',
			right: 'd',
			crouch: 'z',
			mouse_sensitivity: 100.0,
			mouse_stutter_filter: 1,
			mouse_smoothing: 0.0,
		}
	}
}

#[derive(Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlayerOpts {
	pub name: String,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SoundOpts {
	pub enabled: bool,
	pub music: bool,
}

impl Default for SoundOpts {
	fn default() -> Self {
		Self { enabled: true, music: false }
	}
}

#[derive(Deserialize, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct NetworkOpts {
	pub servers: Vec<String>,
}

#[derive(Deserialize, Clone, Serialize, Debug, EguiInspect)]
#[serde(default)]
pub struct DebugOpts {
	pub pause_all_systems: bool,
	pub tick_plankton: bool,
	//pub enable_animation: bool,
	//pub enable_congestion: bool,
	//pub navigation_overlay: bool,
	//pub congestion_overlay: bool,
	//pub reservation_overlay: bool,
	//pub pathfinding_overlay: bool,
	//pub taskman_empty_hands: bool,
	//pub taskman_give_work: bool,
	//pub expire_factory_reservations: bool,
	//pub tick_factories: bool,
	//pub god_mode: bool,
}

impl Default for DebugOpts {
	fn default() -> Self {
		Self {
			pause_all_systems: false,
			tick_plankton: true,
		}
	}
}

// User settings for graphics quality.
#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct GraphicsOpts {
	pub width: u32,
	pub height: u32,
	pub fullscreen: bool,
	pub anisotropy: u8,
	pub texture_resolution: u32,
	pub normal_maps: bool,
	pub msaa: bool,
	pub textures: bool,
	pub mipmaps: bool,
	pub trilinear: bool,
	pub lightmap_nearest: bool,
	pub vsync: bool,
	pub shadows: bool,
	#[serde(default = "default_true")]
	pub hud: bool,
}

fn default_true() -> bool {
	true
}

impl GraphicsOpts {
	pub fn msaa_sample_count(&self) -> u32 {
		// currently WGPU only supports 1 or 4 samples (https://github.com/gfx-rs/wgpu/issues/1832)
		match self.msaa {
			true => 4,
			false => 1,
		}
	}

	pub fn anisotropy_clamp(&self) -> u16 {
		// must be at least 1.
		match self.anisotropy {
			0 | 1 => 1,
			2 | 4 | 8 | 16 => self.anisotropy as u16,
			_ => {
				log::error!("invalid anisotropy: {}", self.anisotropy);
				1
			} // invalid. TODO: check on start-up
		}
	}
}

impl Default for GraphicsOpts {
	fn default() -> Self {
		Self {
			width: 1280,
			height: 768,
			fullscreen: false,
			msaa: false,
			anisotropy: 16,
			texture_resolution: 512,
			normal_maps: true,
			textures: true,
			mipmaps: false,
			trilinear: false,
			lightmap_nearest: false,
			vsync: true,
			shadows: true,
			hud: true,
		}
	}
}
