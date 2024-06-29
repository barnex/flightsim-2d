use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct EguiCanvas {
	// resources stored in `Renderer`.
}

struct CanvasEguiCallback {
	scenegraph: Scenegraph,
	viewport_size: vec2u,
}

impl EguiCanvas {
	pub fn new(cc: &eframe::CreationContext<'_>, opts: &GraphicsOpts) -> Self {
		// Get the WGPU render state from the eframe creation context. This can also be retrieved
		// from `eframe::Frame` when you don't have a `CreationContext` available.
		let render_state = cc.wgpu_render_state.as_ref().expect("WGPU enabled"); // <<< graphics context: adapter, device, queue, ...

		let renderer = Renderer::new(&render_state.device, &render_state.queue, render_state.target_format, opts);

		// Because the graphics pipeline must have the same lifetime as the egui render pass,
		// instead of storing the pipeline in our `Canvas` struct, we insert it into the
		// `paint_callback_resources` type map, which is stored alongside the render pass.
		render_state.renderer.write().callback_resources.insert(renderer);

		Self {}
	}

	pub fn paint(&mut self, ui: &mut egui::Ui, rect: Rect, scenegraph: Scenegraph) -> egui::layers::ShapeIdx {
		let viewport_size = vec2(rect.width(), rect.height()).as_u32();
		ui.painter().add(egui_wgpu::Callback::new_paint_callback(rect, CanvasEguiCallback { scenegraph, viewport_size }))
	}
}

impl CallbackTrait for CanvasEguiCallback {
	fn prepare(
		&self,
		_device: &wgpu::Device,
		_queue: &wgpu::Queue,
		_screen_descriptor: &egui_wgpu::ScreenDescriptor,
		egui_encoder: &mut wgpu::CommandEncoder,
		callback_resources: &mut egui_wgpu::CallbackResources,
	) -> Vec<wgpu::CommandBuffer> {
		if let Some(renderer) = callback_resources.get_mut::<Renderer>() {
			renderer.prepare(egui_encoder, &self.scenegraph, self.viewport_size)
		} else {
			log::error!("❌ prepare: no resources");
			vec![]
		}
	}

	fn paint<'rpass>(&'rpass self, _info: egui::PaintCallbackInfo, render_pass: &mut wgpu::RenderPass<'rpass>, callback_resources: &'rpass egui_wgpu::CallbackResources) {
		if let Some(renderer) = callback_resources.get::<Renderer>() {
			renderer.paint(render_pass)
		} else {
			log::error!("❌ paint: no resources");
		}
	}

	fn finish_prepare(
		&self,
		_device: &wgpu::Device,
		_queue: &wgpu::Queue,
		_egui_encoder: &mut wgpu::CommandEncoder,
		_callback_resources: &mut egui_wgpu::CallbackResources,
	) -> Vec<wgpu::CommandBuffer> {
		vec![]
	}
}
