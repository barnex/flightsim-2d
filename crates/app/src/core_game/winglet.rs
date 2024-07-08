use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect, Setters, Copy)]
pub struct Winglet {
	pub pos: vec2f,
	pub pitch: f32,
	pub drag_factor: f32,
	pub lift_to_drag: f32,
}
