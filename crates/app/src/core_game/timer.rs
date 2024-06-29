use crate::*;

#[derive(Clone, Serialize, Deserialize, Default, Debug, EguiInspect)]
pub struct Timer {
	pub alarm_frame: Mut<u32>,
}

impl Timer {
	pub fn new(alarm_frame: u32) -> Self {
		Self { alarm_frame: alarm_frame.into() }
	}

	pub fn set_alarm(&self, frame: u32, duration: u32) {
		self.alarm_frame.set(frame + duration)
	}

	#[must_use]
	pub fn just_finished(&self, frame: u32) -> bool {
		frame == self.alarm_frame.get()
	}

	#[must_use]
	pub fn finished(&self, frame: u32) -> bool {
		frame >= self.alarm_frame.get()
	}
}
