use crate::prelude::*;

use egui_inspect::EguiInspect;
use egui_plot::Polygon;

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
	ui_state: UiState,

	gs: GameState,
	update_scenegraph: bool,
	#[serde(skip)]
	scenegraph: Scenegraph,

	canvas: EguiCanvas,

	plot_axes: [[usize; 2]; NUM_PLOTS],
}

const NUM_PLOTS: usize = 3;

#[derive(Serialize, Deserialize, Default, Debug, EguiInspect)]
struct UiState {
	gamestate_open: bool,
	scenegraph_open: bool,
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
			gs: default(),
			ui_state: UiState::default(),
			scenegraph: default(),
			update_scenegraph: true,
			plot_axes: [[1, 2], [0, 1], [0, 2]],
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
		self.gs.tick();

		self.top_panel(ctx);
		self.left_panel(ctx);
		self.right_panel(ctx);
		self.bottom_panel(ctx);
		self.central_panel(ctx);

		ctx.request_repaint();
	}

	fn top_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			ui.horizontal(|ui| {
				egui::menu::bar(ui, |ui| {
					ui.button("❌").on_hover_text("Quit").clicked().then(|| ctx.send_viewport_cmd(egui::ViewportCommand::Close));
					self.dark_mode_switch(ui);
					Self::toggle_button(ui, &mut self.gs.debug.pause_all_systems, "⏸", "Pause all systems");
					if ui.button("⏰").on_hover_text("tick").clicked() {
						self.gs.record_plot();
						self.gs.debug.pause_all_systems = true;
					}
					Self::toggle_button(ui, &mut self.gs.debug.force_record_plots, "📈", "Record plot even when paused");
					Self::toggle_button(ui, &mut self.gs.camera_follows, "🎥", "Camera follows airplane");
					Self::toggle_button(ui, &mut self.ui_state.gamestate_open, "🔎", "Show gamestate");
					Self::toggle_button(ui, &mut self.ui_state.scenegraph_open, "🎬", "Show scenegraph");
					Self::toggle_button(ui, &mut self.ui_state.commands_open, "$>", "Show commands");
				});
				ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(&self.gs.fps_label));
			})
		});
	}

	fn right_panel(&mut self, ctx: &egui::Context) {
		egui::SidePanel::right("right_panel").min_width(320.0).show(ctx, |ui| {
			let plane = &mut self.gs.plane;
			let default = Plane::default();

			ui.heading("controls");
			let a = 30.0 * DEG;
			slider(ui, "◀ ▶throttle", "N", 0.0..=(plane.max_propeller_force), &mut plane.propeller_force);
			slider(ui, "↕elevator", "rad", -a..=a, &mut plane.elevator.pitch);
			ui.label("(or use arrow keys)");
			ui.horizontal(|ui| {
				ui.toggle_value(&mut self.gs.debug.pause_all_systems, "pause");
				if ui.button("❌ reset").clicked() {
					plane.body.velocity = vec::ZERO;
					plane.body.rot_velocity = 0.0;
					plane.body.rotation = 0.0;
					plane.body.position = Plane::default().body.position;
					self.gs.plotter.clear();
					self.gs.frame = 0;
				}
			});

			ui.heading("kinematics");
			inspect_vec2(ui, "position", "m", 1, &mut plane.body.position);
			inspect_vec2(ui, "velocity", "m/s", 1, &mut plane.body.velocity);
			inspect_vec2(ui, "acceleration", "m/s²", 1, &mut plane.body.acceleration);
			set_angle(ui, "pitch", -PI..=PI, 0.0, &mut plane.body.rotation);
			inspect_value(ui, "rot. velocity", "rad/s", 2, &mut plane.body.rot_velocity);
			inspect_value(ui, "rot. accell", "rad/s²", 2, &mut plane.body.rot_accel);

			ui.heading("✈aircraft design");
			set_quantity(ui, "mass", "kg", 0.0..=10000.0, 10.0, default.body.mass, &mut plane.body.mass);
			set_quantity(ui, "rot. intertia", "kg m²", 0.0..=10000.0, 10.0, default.body.rot_inertia, &mut plane.body.rot_inertia);
			set_quantity(ui, "gravity", "N/kg", 0.0..=10.0, 0.01, default.gravity, &mut plane.gravity);
			set_quantity(ui, "fuselage drag", "N/√(m/s)", 0.0..=5.0, 0.001, default.body_drag, &mut plane.body_drag);
			set_quantity(ui, "max propeller", "N", 0.0..=5000.0, 10.0, default.max_propeller_force, &mut plane.max_propeller_force);

			ui.strong("✈wings");

			set_vec2(ui, "wings pos", "m", -10.0..=10.0, 0.01, default.wings.pos, &mut plane.wings.pos);
			set_angle(ui, "wings pitch", -a..=a, default.wings.pitch, &mut plane.wings.pitch);
			set_quantity(ui, "wings drag", "N/√(m/s)", 0.0..=10.0, 0.001, default.wings.drag_factor, &mut plane.wings.drag_factor);
			set_quantity(ui, "wings l2d", "", 0.0..=20.0, 0.1, default.wings.lift_to_drag, &mut plane.wings.lift_to_drag);

			ui.strong("↕elevator");
			set_vec2(ui, "elevator pos", "m", -10.0..=10.0, 0.01, default.elevator.pos, &mut plane.elevator.pos);
			set_angle(ui, "elevator pitch", -a..=a, default.elevator.pitch, &mut plane.elevator.pitch);
			set_quantity(ui, "elevator drag", "N/√(m/s)", 0.0..=5.0, 0.001, default.elevator.drag_factor, &mut plane.elevator.drag_factor);
			set_quantity(ui, "elevator l2d", "", 0.0..=20.0, 0.1, default.elevator.lift_to_drag, &mut plane.elevator.lift_to_drag);

			ui.strong("💿wheels");
			set_vec2(ui, "wheel1 pos", "m", -10.0..=10.0, 0.01, default.wheels[0], &mut plane.wheels[0]);
			set_vec2(ui, "wheel2 pos", "m", -10.0..=10.0, 0.01, default.wheels[1], &mut plane.wheels[1]);

			if ui.button("❌ reset design").clicked() {
				*plane = Plane::default();
			}

			ui.heading("camera");
			ui.checkbox(&mut self.gs.camera_follows, "follows aircraft");
			set_quantity(ui, "zoom", "", 0.5..=64.0, 32.0, 0.5, &mut self.gs.camera.zoom);
			set_quantity(ui, "follow speed", "", 0.01..=1.0, 0.01, 0.3, &mut self.gs.camera_follow_speed);
			ui.checkbox(&mut plane.draw_forces, "draw forces");
			set_quantity(ui, "timewarp", "s/s", 1..=100, 1.0, 1, &mut self.gs.debug.timepassage);
		});
	}

	fn bottom_panel(&mut self, ctx: &egui::Context) {
		egui::TopBottomPanel::bottom("bottom_panel").min_height(120.0).show(ctx, |ui| {
			ui.columns(4, |cols| {
				let plane = &self.gs.plane;
				let ui = &mut cols[0];
				ui.heading(&format!("airspeed: {:.0} m/s", plane.body.velocity.len()));
				ui.heading(&format!("pitch: {:+.1}°", plane.body.rotation / DEG));
				ui.heading(&format!("climb: {:+.1} m/s", plane.body.velocity.y()));
				ui.heading(&format!("altitude: {:.0} m", plane.body.position.y()));
				ui.heading(&format!("AOA: {:+03.1}°", plane.winglet_aoa(&plane.wings) / DEG));

				ui.label(&format!("elevator AOA: {:+02.1}°", plane.winglet_aoa(&plane.elevator) / DEG));
				ui.label(&format!("elevator lift: {:.1} N", plane.winglet_lift(&plane.elevator).len()));
				ui.label(&format!("elevator drag: {:.1} N", plane.winglet_induced_drag(&plane.elevator).len()));

				self.plot(&mut cols[1], 0);
				self.plot(&mut cols[2], 1);
				self.plot(&mut cols[3], 2);
			});
			ui.hyperlink("http://github.com/barnex/flightsim-2d");
		});
	}

	fn plot(&mut self, ui: &mut egui::Ui, plot_i: usize) {
		use egui_plot::{Line, PlotPoints};
		ui.horizontal(|ui| {
			for (sel, id) in iter::zip(self.plot_axes[plot_i].iter_mut(), [(plot_i, 0), (plot_i, 1)]) {
				egui::ComboBox::from_id_source(id).selected_text(&self.gs.plotter.labels[*sel]).show_ui(ui, |ui| {
					for (i, label) in self.gs.plotter.labels.iter().enumerate() {
						ui.selectable_value(sel, i, label);
					}
				});
			}
			if ui.button("❌").clicked() {
				self.gs.plotter.clear()
			}
		});

		let line = self.gs.plotter.line(self.plot_axes[plot_i][0], self.plot_axes[plot_i][1]);
		egui_plot::Plot::new("example_plot")
			.show_axes(true)
			.allow_drag(true)
			.allow_zoom(true)
			.allow_scroll(true)
			.center_x_axis(false)
			.center_x_axis(false)
			.height(300.0)
			.x_axis_label(&self.gs.plotter.labels[self.plot_axes[plot_i][0]])
			.y_axis_label(&self.gs.plotter.labels[self.plot_axes[plot_i][1]])
			.show(ui, |plot_ui| {
				plot_ui.line(line);
			});
	}

	fn left_panel(&mut self, ctx: &egui::Context) {
		// borrow checker dance since ui_state window controls it's open openness.
		if self.ui_state.gamestate_open {
			egui::SidePanel::left("left_panel").show(ctx, |ui| {
				if ui.button("❌ reset").clicked() {
					self.gs = default();
				}
				self.gs.inspect_mut("gamestate", ui);
			});
		}
	}

	fn central_panel(&mut self, ctx: &egui::Context) {
		egui::CentralPanel::default().frame(egui::Frame::default()).show(ctx, |ui| {
			self.scenegraph_window(ctx);
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
				self.scenegraph.clear();
				self.gs.draw_on(&mut self.scenegraph);
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
	fn context_menu(&mut self, ctx: &egui::Context, ui: &mut Ui) {}

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

	fn auto_save_interval(&self) -> Duration {
		Duration::MAX // effectively disable
	}

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

fn with_labels<X>(ui: &mut egui::Ui, prefix: &str, suffix: &str, add_contents: impl FnOnce(&mut egui::Ui) -> X) {
	ui.horizontal(|ui| {
		ui.label(prefix);
		add_contents(ui);
		ui.label(suffix);
	});
}

fn set_quantity<T>(ui: &mut egui::Ui, prefix: &str, suffix: &str, range: std::ops::RangeInclusive<T>, speed: f64, default: T, value: &mut T)
where
	T: egui::emath::Numeric + PartialEq + Copy,
{
	ui.horizontal(|ui| {
		ui.label(prefix);
		ui.add(egui::DragValue::new(value).speed(speed).clamp_range(range));
		ui.label(suffix);
		if *value != default && ui.button("❌ reset").on_hover_text("reset to default").clicked() {
			*value = default;
		}
	});
}

fn set_angle(ui: &mut egui::Ui, prefix: &str, range: std::ops::RangeInclusive<f32>, default: f32, value: &mut f32) {
	ui.horizontal(|ui| {
		ui.label(prefix);
		ui.drag_angle(value);
		*value = value.clamp(*range.start(), *range.end());
		if *value != default && ui.button("❌ reset").on_hover_text("reset to default").clicked() {
			*value = default;
		}
	});
}

fn set_vec2(ui: &mut egui::Ui, prefix: &str, suffix: &str, range: std::ops::RangeInclusive<f64>, speed: f64, default: vec2f, value: &mut vec2f) {
	ui.horizontal(|ui| {
		ui.label(prefix);
		let vec([x, y]) = value;
		ui.add(egui::DragValue::new(x).speed(speed).clamp_range(range.clone()));
		ui.add(egui::DragValue::new(y).speed(speed).clamp_range(range.clone()));
		ui.label(suffix);
		if *value != default && ui.button("❌ reset").on_hover_text("reset to default").clicked() {
			*value = default;
		}
	});
}

fn inspect_value<T>(ui: &mut egui::Ui, prefix: &str, suffix: &str, decimals: usize, value: &mut T)
where
	T: egui::emath::Numeric + PartialEq + Copy + Default,
{
	ui.horizontal(|ui| {
		ui.label(prefix);
		ui.add(egui::DragValue::new(value).fixed_decimals(decimals).speed(10.0f32.powf(-(decimals as f32))));
		ui.label(suffix);
		if ui.button("❌").clicked() {
			*value = T::default();
		}
	});
}

fn inspect_values2<T>(ui: &mut egui::Ui, prefix: &str, suffix: &str, decimals: usize, v1: &mut T, v2: &mut T)
where
	T: egui::emath::Numeric + PartialEq + Copy + Default,
{
	ui.horizontal(|ui| {
		ui.label(prefix);
		ui.add(egui::DragValue::new(v1).fixed_decimals(decimals));
		ui.add(egui::DragValue::new(v2).fixed_decimals(decimals));
		ui.label(suffix);
		if ui.button("❌").clicked() {
			*v1 = T::default();
			*v2 = T::default();
		}
	});
}

fn inspect_vec2(ui: &mut egui::Ui, prefix: &str, suffix: &str, decimals: usize, v: &mut vec2f) {
	let vec([x, y]) = v;
	inspect_values2(ui, prefix, suffix, decimals, x, y);
}

fn slider<T>(ui: &mut egui::Ui, prefix: &str, suffix: &str, range: std::ops::RangeInclusive<T>, value: &mut T)
where
	T: egui::emath::Numeric + PartialEq + Copy,
{
	ui.horizontal(|ui| {
		ui.label(prefix);
		ui.add(egui::Slider::new(value, range));
		ui.label(suffix);
	});
}
