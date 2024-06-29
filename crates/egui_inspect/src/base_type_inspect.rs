use crate::InspectNumber;
use crate::InspectString;
use crate::*;
use egui::Ui;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

macro_rules! impl_inspect_float {
    ($($t:ty),+) => {
        $(
            impl crate::InspectNumber for $t {
                fn inspect_with_slider(&mut self, label: &str, ui: &mut egui::Ui, min: f32, max: f32) {
                    ui.horizontal(|ui| {
                        ui.label(label);
                        ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                    });
                }
                fn inspect_with_drag_value(&mut self, label: &str, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
                        ui.label(label);
                        ui.add(egui::DragValue::new(self).speed(0.1));
                    });
                }
            }

            impl crate::EguiInspect for $t {
                fn inspect(&self, label: &str, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
                        ui.label(label);
                        ui.label(self.to_string());
                    });
                }
                fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
                    self.inspect_with_drag_value(label, ui);
                }
            }
        )*
    }
}

macro_rules! impl_inspect_int {
    ($($t:ty),+) => {
        $(
        impl crate::InspectNumber for $t {
            fn inspect_with_slider(&mut self, label: &str, ui: &mut egui::Ui, min: f32, max: f32) {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(egui::Slider::new(self, (min as $t)..=(max as $t)));
                });
            }
            fn inspect_with_drag_value(&mut self, label: &str, ui: &mut egui::Ui) {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.add(egui::DragValue::new(self));
                });
            }
        }

        impl crate::EguiInspect for $t {
            fn inspect(&self, label: &str, ui: &mut egui::Ui) {
                ui.horizontal(|ui| {
                    ui.label(label);
                    ui.label(self.to_string());
                });
            }
            fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
                self.inspect_with_drag_value(label, ui);
            }
        }
        )*
    }
}

impl_inspect_float!(f32, f64);

impl_inspect_int!(i8, u8);
impl_inspect_int!(i16, u16);
impl_inspect_int!(i32, u32);
impl_inspect_int!(i64, u64);
impl_inspect_int!(isize, usize);

impl crate::EguiInspect for &'static str {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			ui.label(self.to_string());
		});
	}
}

impl crate::EguiInspect for String {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			ui.label(self);
		});
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.inspect_mut_singleline(label, ui);
	}
}

impl crate::InspectString for String {
	fn inspect_mut_multiline(&mut self, label: &str, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			ui.text_edit_multiline(self);
		});
	}

	fn inspect_mut_singleline(&mut self, label: &str, ui: &mut Ui) {
		ui.horizontal(|ui| {
			ui.label(label);
			ui.text_edit_singleline(self);
		});
	}
}

impl crate::EguiInspect for bool {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.add_enabled(false, egui::Checkbox::new(&mut self.clone(), label));
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		ui.checkbox(self, label);
	}
}

impl<T: crate::EguiInspect, const N: usize> crate::EguiInspect for [T; N] {
	fn inspect(&self, label: &str, ui: &mut Ui) {
		ui.label(label);
		ui.horizontal(|ui| {
			for (i, item) in self.iter().enumerate() {
				item.inspect("", ui);
			}
		});
	}

	fn inspect_mut(&mut self, label: &str, ui: &mut Ui) {
		ui.label(label);
		ui.horizontal(|ui| {
			for (i, item) in self.iter_mut().enumerate() {
				item.inspect_mut("", ui);
			}
		});
	}
}

impl<T: EguiInspect> EguiInspect for Vec<T> {
	fn inspect(&self, label: &str, ui: &mut Ui) {
		let hash = self.as_ptr();
		inspect_iter(ui, label, self.iter(), hash)
	}

	fn inspect_mut(&mut self, label: &str, ui: &mut Ui) {
		let hash = self.as_ptr();
		inspect_iter_mut(ui, label, self.iter_mut(), hash)
	}
}

impl<T: EguiInspect> EguiInspect for &[T] {
	fn inspect(&self, label: &str, ui: &mut Ui) {
		let hash = self.as_ptr();
		inspect_iter(ui, label, self.iter(), hash)
	}
}

impl<T: EguiInspect> EguiInspect for &mut [T] {
	fn inspect(&self, label: &str, ui: &mut Ui) {
		let hash = self.as_ptr();
		inspect_iter(ui, label, self.iter(), hash)
	}

	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		let hash = self.as_ptr();
		inspect_iter_mut(ui, label, self.iter_mut(), hash)
	}
}

impl<T: EguiInspect> EguiInspect for Box<[T]> {
	fn inspect(&self, label: &str, ui: &mut Ui) {
		inspect_iter(ui, label, self.iter(), self.as_ptr())
	}
}

impl<T: EguiInspect> EguiInspect for Rc<[T]> {
	fn inspect(&self, label: &str, ui: &mut Ui) {
		inspect_iter(ui, label, self.iter(), self.as_ptr())
	}
}

impl<T: EguiInspect> EguiInspect for RefCell<T> {
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.borrow().inspect(label, ui);
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.borrow_mut().inspect_mut(label, ui)
	}
}

/// Egui Inspect a collection (Vec, &[T], Box<[T]>, ...) of values.
/// Annoyingly, a `hash` unique to the collection is needed to generate unique Egui labels.
/// `self.as_ptr()` works well.
fn inspect_iter<'i, I, T, H>(ui: &mut egui::Ui, label: &str, v: I, hash: H)
where
	I: ExactSizeIterator<Item = &'i T> + 'i,
	T: EguiInspect + 'i,
	H: std::hash::Hash,
{
	// 👇 Empty collections tend to have colliding hashes (`as_ptr == NULL`),
	// so use Label (does not require hash) instead of CollapsingHeader.
	if v.len() == 0 {
		ui.label(format!("{label} (len {})", v.len()));
		return;
	}

	// 👇 hacky hash to (hopefully) ensure unique egui IDs.
	let hash = (label, v.len(), hash);
	egui::CollapsingHeader::new(format!("{label} (len {})", v.len())).id_source(hash).default_open(false).show(ui, |ui| {
		for (i, item) in v.enumerate() {
			egui::CollapsingHeader::new(format!("{label} [{i}]")).show(ui, |ui| {
				item.inspect("", ui);
			});
		}
	});
}

fn inspect_iter_mut<'i, I, T, H>(ui: &mut egui::Ui, label: &str, v: I, hash: H)
where
	I: ExactSizeIterator<Item = &'i mut T> + 'i,
	T: EguiInspect + 'i,
	H: std::hash::Hash,
{
	// 👇 Empty collections tend to have colliding hashes (`as_ptr == NULL`),
	// so use Label (does not require hash) instead of CollapsingHeader.
	if v.len() == 0 {
		ui.label(format!("{label} (len {})", v.len()));
		return;
	}

	// 👇 hacky hash to (hopefully) ensure unique egui IDs.
	let hash = (label, v.len(), hash);
	egui::CollapsingHeader::new(format!("{label} (len {})", v.len())).id_source(hash).default_open(false).show(ui, |ui| {
		for (i, item) in v.enumerate() {
			egui::CollapsingHeader::new(format!("{label} [{i}]")).show(ui, |ui| {
				item.inspect_mut("", ui);
			});
		}
	});
}

impl<T> EguiInspect for Option<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		match self {
			Some(v) => v.inspect(label, ui),
			None => {
				ui.horizontal(|ui| {
					ui.label(label);
					ui.label("None");
				});
			}
		}
	}

	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		match self {
			Some(v) => v.inspect_mut(label, ui),
			None => {
				ui.horizontal(|ui| {
					ui.label(label);
					ui.label("None");
				});
			}
		}
	}
}

impl<A, B> EguiInspect for (A, B)
where
	A: EguiInspect,
	B: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.0.inspect(&(label.to_string() + ".0"), ui);
		self.1.inspect(&(label.to_string() + ".1"), ui);
	}

	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.0.inspect_mut(&(label.to_string() + ".0"), ui);
		self.1.inspect_mut(&(label.to_string() + ".1"), ui);
	}
}

impl<T> EguiInspect for std::rc::Rc<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.as_ref().inspect(label, ui)
	}
}
