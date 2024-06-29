use crate::prelude::*;

use egui_inspect::EguiInspect;
mod commands;

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
	ui_state: UiState,

	gs: GameState,
	update_scenegraph: bool,
	#[serde(skip)]
	scenegraph: Scenegraph,

	canvas: EguiCanvas,
}

#[derive(Serialize, Deserialize, Default, Debug, EguiInspect)]
struct UiState {
	gamestate_open: bool,
	scenegraph_open: bool,
	ui_state_open: bool,
	scope_open: bool,
	profiler_open: bool,
	stats_open: bool,
	commands_open: bool,
	dark_mode: bool,
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		let settings = Settings::todo_load_settings();
		let s = Self::try_restore(cc, &settings).unwrap_or_else(|| Self::default(cc, &settings));
		// egui does not restore dark mode for some reason
		if s.ui_state.dark_mode {
			cc.egui_ctx.set_visuals(egui::Visuals::dark());
		}
		s
	}

	fn default(cc: &eframe::CreationContext<'_>, settings: &Settings) -> Self {
		Self {
			canvas: EguiCanvas::new(cc, &settings.graphics),
			//gs: GameState::default(),
			gs: airplaine_gamestate(),
			ui_state: UiState::default(),
			scenegraph: default(),
			update_scenegraph: true,
		}
	}

	// Load previous app state (if any).
	fn try_restore(cc: &eframe::CreationContext<'_>, settings: &Settings) -> Option<Self> {
		log::info!("egui storage ok: {}", cc.storage.is_some());
		let storage = cc.storage?;

		// deserialize from base64(zip(rmp))
		let enc64 = storage.get_string(eframe::APP_KEY)?;
		let restored = rmp_serde::from_read(GzDecoder::new(base64::read::DecoderReader::new(io::Cursor::new(enc64), &BASE64_STANDARD)))
			.inspect_err(|e| log::error!("{e:#}"))
			.ok()?;

		Some(Self {
			canvas: EguiCanvas::new(cc, &settings.graphics), // 👈 hack to initialize wgpu resources
			..restored
		})
	}

	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		profiler::restart("frame time");

		self.gs.tick();

		profiler::scope("egui", || {
			self.top_panel(ctx);
			self.left_panel(ctx);
			self.right_panel(ctx);
			self.central_panel(ctx);
		});

		ctx.request_repaint();
	}

	fn top_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				egui::menu::bar(ui, |ui| {
					ui.button("❌").on_hover_text("Quit").clicked().then(|| ctx.send_viewport_cmd(egui::ViewportCommand::Close));
					self.dark_mode_switch(ui);
					Self::toggle_button(ui, &mut self.gs.debug.pause_all_systems, "⏸", "Pause all systems");
					Self::toggle_button(ui, &mut self.ui_state.gamestate_open, "🔎", "Show gamestate");
					Self::toggle_button(ui, &mut self.ui_state.scenegraph_open, "🎬", "Show scenegraph");
					Self::toggle_button(ui, &mut self.ui_state.ui_state_open, "↗", "Show UI state");
					Self::toggle_button(ui, &mut self.ui_state.commands_open, "$>", "Show commands");
					Self::toggle_button(ui, &mut self.ui_state.scope_open, "~", "Show scope");
					Self::toggle_button(ui, &mut self.ui_state.profiler_open, "⏳", "Show profiler");
					Self::toggle_button(ui, &mut self.ui_state.stats_open, "📈", "Show stats");
				});
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(&self.gs.fps_label));
			})
		});
	}

	fn right_panel(&mut self, ctx: &egui::Context) {
		if let Some(plane) = &mut self.gs.plane {
			egui::SidePanel::right("right_panel").min_width(320.0).show(ctx, |ui| {
				ui.vertical(|ui| {
					plane.inspect_mut("plane", ui);
					// ... nice controls
				})
			});
		} else if let Some(id) = self.gs.selected_crab() {
			if let Some(crab) = self.gs.crablets.get_mut(id) {
				egui::SidePanel::right("right_panel").min_width(250.0).show(ctx, |ui| {
					ui.vertical(|ui| {
						crab.inspect_mut(&format!("crab {id:?}"), ui);
						Self::flipper_controls(ui, crab);
					})
				});
			}
		}
	}

	fn crab_panel(&mut self, ctx: &egui::Context) {
		if let Some(id) = self.gs.selected_crab() {
			if let Some(crab) = self.gs.crablets.get_mut(id) {
				egui::SidePanel::right("right_panel").min_width(250.0).show(ctx, |ui| {
					ui.vertical(|ui| {
						crab.inspect_mut(&format!("crab {id:?}"), ui);
						Self::flipper_controls(ui, crab);
					})
				});
			}
		}
	}

	fn flipper_controls(ui: &mut egui::Ui, crab: &mut Crablet) {
		let mut flippers = [0.0; 2];
		let mut both = 0.0;
		ui.label("flippers");
		ui.horizontal(|ui| {
			ui.add(egui::Slider::new(&mut flippers[1], -1.0..=1.0).vertical().show_value(false));
			ui.add(egui::Slider::new(&mut both, -1.0..=1.0).vertical().show_value(false));
			ui.add(egui::Slider::new(&mut flippers[0], -1.0..=1.0).vertical().show_value(false));
		});
		flippers[0] += both;
		flippers[1] += both;

		let mut flipper_power = crab.flipper_power();
		let a = 0.1;
		flipper_power[0] = (1.0 - a) * flipper_power[0] + a * flippers[0];
		flipper_power[1] = (1.0 - a) * flipper_power[1] + a * flippers[1];
		crab.set_flipper_power(flipper_power);
	}

	//fn brain_view(ui: &mut egui::Ui, brain: &mut Net) {
	//	for i in 0..brain.layers.len() - 1 {
	//		Self::layer_view(ui, &mut brain.layers[i]);
	//		Self::func_view(ui, &mut brain.functions[i]);
	//	}
	//	let last = brain.layers.len() - 1;
	//	Self::layer_view(ui, &mut brain.layers[last]);
	//}

	//fn layer_view(ui: &mut egui::Ui, layer: &mut [f32]) {
	//	ui.horizontal(|ui|{
	//		for v in layer{
	//			ui.add(egui::DragValue::new(v));
	//		}
	//	});
	//}

	//fn func_view(ui: &mut egui::Ui, func: &mut Func) {
	//	let n = func.weights.len();
	//	ui.horizontal(|ui|{
	//		for i in 0..n{
	//			ui.add(egui::DragValue::new(&mut func.weights[i]));
	//		}
	//	});
	//}

	fn left_panel(&mut self, ctx: &egui::Context) {
		// borrow checker dance since ui_state window controls it's open openness.
		if self.ui_state.gamestate_open {
			egui::SidePanel::left("left_panel").show(ctx, |ui| {
				if ui.button("❌ reset").clicked() {
					self.gs = default();
				}
				if ui.button("✈ airplane").clicked() {
					self.gs = airplaine_gamestate()
				}
				self.gs.inspect_mut("gamestate", ui);
			});
		}
	}

	fn central_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().frame(egui::Frame::default()).show(ctx, |ui| {
			self.scenegraph_window(ctx);
			self.scope_window(ctx);
			self.profiler_window(ctx);
			self.stats_window(ctx);
			self.command_window(ctx);
			self.canvas(ctx, ui);
		});
	}

	// Main Canvas: Game world is rendered here.
	fn canvas(&mut self, ctx: &egui::Context, ui: &mut Ui) {
		let mut context_menu_open = false; // hack to avoid Canvas click events when context menu is open
								   // Else, when we click the context menu, we *also* click the underlying canvas.
		let response = egui::Frame {
			fill: egui::Color32::from_gray(128),
			..default()
		}
		.show(ui, |ui| {
			let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::focusable_noninteractive());
			self.gs.camera.viewport_size_pix = vec2(rect.width(), rect.height()).as_u32();
			if self.update_scenegraph {
				profiler::scope("update_scenegraph", || {
					self.scenegraph.clear();
					self.gs.draw_on(&mut self.scenegraph);
				});
			}
			self.canvas.paint(ui, rect, self.scenegraph.clone());

			// show context menu on right-click
			let _menu_resp = response.context_menu(|ui| {
				context_menu_open = true;
				self.context_menu(ctx, ui);
			});
			response
		});

		let rect = response.inner.rect;
		// Hack to avoid Canvas clicks when context menu is open.
		// `ctx.is_using_pointer()`, `ctx.wants_pointer_input()` don't have the desired behavior.
		if let Some(mouse_pos) = ctx.input(|inputs| inputs.pointer.interact_pos()) {
			if !context_menu_open && rect.contains(mouse_pos) {
				ctx.input(|inputs| self.gs.inputs.record(rect.left_top().into(), inputs));
			}
		}
	}

	// Context menu (right-click) in Main Canvas.
	fn context_menu(&mut self, ctx: &egui::Context, ui: &mut Ui) {
		// close context menu
		//let mut clicked = false;

		ui.menu_button("TODO context", |ui| ui.label("coming soon..."));

		if ui.button("click").clicked() {}
		if ui.button("clack").clicked() {}
		if ui.button("clock").clicked() {}

		//if clicked {
		//	ui.close_menu()
		//}

		//if self.ui_state.close_context_menu_on_click {
		//	if ctx.input(|inputs| inputs.pointer.any_click()) {
		//		ui.close_menu();
		//	}
		//}

		//self.ui_state.close_context_menu_on_click = ctx.is_context_menu_open(); // <<<<<<<<<<< remove
	}

	//fn ui_state_window(&mut self, ctx: &egui::Context) {
	//	// borrow checker dance since ui_state window controls it's open openness.
	//	let mut inspector_open = self.ui_state.ui_state_open;
	//	egui::Window::new("↗ ui state") //.
	//		.open(&mut inspector_open)
	//		.show(ctx, |ui| {
	//			self.ui_state.inspect_mut("ui_state", ui);
	//		});
	//	self.ui_state.ui_state_open = inspector_open;
	//}

	fn profiler_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("⏳ profiler") //.
			.open(&mut self.ui_state.profiler_open)
			.show(ctx, |ui| ui.add(egui::Label::new(profiler::to_string())));
	}

	fn stats_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("📈 stats") //.
			.open(&mut self.ui_state.stats_open)
			.show(ctx, |ui| {
				egui::Grid::new("some_unique_id").show(ui, |ui| {
					ui.label("stat");
					ui.label("frame");
					ui.label("total");
					ui.end_row();
					for (stat, frame, total) in self.gs.stats.iter() {
						ui.label(format!("{stat:?}"));
						ui.label(format!("{frame}"));
						ui.label(format!("{total}"));
						ui.end_row();
					}
					ui.end_row();
				})
			});
	}

	fn scope_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("~ scope") //.
			.open(&mut self.ui_state.scope_open)
			.show(ctx, |ui| ui.add(egui::Label::new(scope::to_string())));
	}

	fn scenegraph_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("🎬 scenegraph") //.
			.open(&mut self.ui_state.scenegraph_open)
			.show(ctx, |ui| {
				ui.checkbox(&mut self.update_scenegraph, "update");
				inspect(ui, "scenegraph", &mut self.scenegraph);
			});
	}

	fn gamestate_window(&mut self, ctx: &egui::Context) {
		egui::Window::new("🔎 state") //.
			.open(&mut self.ui_state.gamestate_open)
			.show(ctx, |ui| {
				if ui.button("❌ reset").clicked() {
					self.gs = default();
				}
				inspect(ui, "state", &mut self.gs);
			});
	}

	fn toggle_button(ui: &mut Ui, state: &mut bool, label: &str, tooltip: &str) {
		ui.add(egui::SelectableLabel::new(*state, label)).on_hover_text(tooltip).clicked().then(|| toggle(state));
	}

	// Like egui's `global_light_dark_mode_switch`, but persists state.
	fn dark_mode_switch(&mut self, ui: &mut Ui) {
		let style: egui::Style = (*ui.ctx().style()).clone();
		let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
		if let Some(visuals) = new_visuals {
			self.ui_state.dark_mode = visuals.dark_mode; // 👈 persist
			ui.ctx().set_visuals(visuals);
		}
	}
}

impl eframe::App for App {
	// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		log::info!("persisting egui state");

		// serialize as base64(gzip(rmp))
		let mut s = base64::write::EncoderStringWriter::new(&BASE64_STANDARD);
		let mut enc = GzEncoder::new(&mut s, flate2::Compression::fast());
		let mut rmp = rmp_serde::Serializer::new(&mut enc).with_struct_map();
		self.serialize(&mut rmp).expect("serialize");
		drop(enc);

		storage.set_string(eframe::APP_KEY, s.into_inner())
	}

	fn on_exit(&mut self) {
		log::info!("exiting");
	}

	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		self.update(ctx, frame);
	}

	//fn auto_save_interval(&self) -> Duration {
	//	Duration::MAX // effectively disable
	//}

	//fn persist_egui_memory(&self) -> bool {
	//	false
	//}
}

fn toggle(ptr: &mut bool) {
	*ptr = !*ptr
}

pub fn inspect(ui: &mut Ui, name: &str, value: &mut dyn EguiInspect) {
	egui::ScrollArea::vertical()
		.scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible)
		.show(ui, |ui| {
			value.inspect_mut(name, ui);
		});
}
