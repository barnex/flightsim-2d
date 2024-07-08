#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use anyhow::Context as _;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> anyhow::Result<()> {
	//env_logger::builder().filter_level(log::LevelFilter::Info).init();
	env_logger::init();

	let native_options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]).with_min_inner_size([300.0, 220.0]),
		vsync: true,
		wgpu_options: eframe::egui_wgpu::WgpuConfiguration {
			..Default::default()
		},
		..Default::default()
	};
	eframe::run_native(
		"Flightsim 2D",
		native_options,
		Box::new(|cc| {
			//egui_extras::install_image_loaders(&cc.egui_ctx);
			Box::new(app::App::new(cc))
		}),
	)
	.context("eframe::run_native")
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
	// Redirect `log` message to `console.log` and friends:
	eframe::WebLogger::init(log::LevelFilter::Debug).ok();

	let web_options = eframe::WebOptions::default();

	wasm_bindgen_futures::spawn_local(async {
		eframe::WebRunner::new()
			.start(
				"the_canvas_id", // hardcode it
				web_options,
				Box::new(|cc| Box::new(app::App::new(cc))),
			)
			.await
			.expect("failed to start eframe");
	});
}
