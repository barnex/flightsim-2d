use crate::prelude::*;

impl App {
	pub fn command_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("$> cmd") //.
			.open(&mut self.ui_state.commands_open)
			.show(ctx, |ui| {
				Self::cmd_button(ui, "tick", || self.gs.manual_tick());
				Self::cmd_button(ui, "spawn crab", || self.gs.spawn_crab());
				Self::cmd_button(ui, "spawn plankton", || self.gs.spawn_plankton());
				//Self::cmd_button(ui, "clear plankton", || self.gs.clear_plankton());
			});
	}

	fn cmd_button(ui: &mut Ui, text: &str, f: impl FnMut()) {
		ui.button(text).clicked().then(f);
	}
}
