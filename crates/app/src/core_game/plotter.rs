use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Plotter {
	pub labels: Vec<String>,
	data: Vec<Vec<f32>>,
	i: usize,
	every: usize,
}

impl Plotter {
	pub fn new(labels: &[&str]) -> Self {
		Self {
			labels: labels.iter().map(|v| v.to_string()).collect(),
			data: labels.iter().map(|_| Vec::new()).collect(),
			i: 0,
			every: 1,
		}
	}

	pub fn clear(&mut self) {
		self.i = 0;
		self.every = 1;
		self.data.iter_mut().for_each(Vec::clear);
	}

	const MAX_LEN: usize = 1024;

	pub fn pushf<F>(&mut self, data: F)
	where
		F: FnOnce() -> Vec<f32>,
	{
		if self.i % self.every == 0 {
			if self.data.len() >= Self::MAX_LEN {
				let mut i = 0;
				self.data.retain(|_| {
					i += 1;
					i % 2 != 0
				});
				self.every *= 2;
			}
			for (dst, &datum) in iter::zip(self.data.iter_mut(), data().iter()) {
				dst.push(datum)
			}
		}

		self.i += 1;
	}

	pub fn line(&self, x: usize, y: usize) -> egui_plot::Line {
		use egui_plot::{Line, PlotPoints};
		egui_plot::Line::new(iter::zip(&self.data[x], &self.data[y]).map(|(x, y)| [*x as f64, *y as f64]).collect::<PlotPoints>())
	}
}

impl fmt::Debug for Plotter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Plotter").field("labels", &self.labels).field("data", &self.data).finish()
	}
}
