use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug, EguiInspect)]
pub struct Brain1 {
	//pub retina: vec2f,
	pub weights: mat2x2f,
	pub biases: vec2f,
	//pub output: vec2f,
}

impl Brain1 {
	pub fn tick(&self, output: &mut vec2f, input: &vec2f) {
		*output = (self.weights * *input) + self.biases;
	}
}

// // TODO: Naming: Architecture, ...
// #[derive(Serialize, Deserialize, Clone, Debug, EguiInspect)]
// pub struct Net {
// 	pub layers: Vec<Vec<f32>>,
// 	pub functions: Vec<Func>,
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug, EguiInspect)]
// pub struct Func {
// 	pub weights: Vec<f32>,
// 	pub op: Op,
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum Op {
// 	Relu,
// }
//
// impl EguiInspect for Op {
// 	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
// 		ui.label(&format!("op: {self:?}"));
// 	}
// }
//
// pub fn a1() -> Net {
// 	Net {
// 		layers: layers(&[2, 2]),
// 		functions: vec![relu(2, 2)],
// 	}
// }
//
// fn layers(num_neurons: &[usize]) -> Vec<Vec<f32>> {
// 	num_neurons.iter().map(|&n| vec![0.0; n]).collect_vec()
// }
//
// fn relu(num_in: usize, num_out: usize) -> Func {
// 	Func {
// 		weights: vec![0.0; (num_in + 1) * num_out],
// 		op: Op::Relu,
// 	}
// }
//
