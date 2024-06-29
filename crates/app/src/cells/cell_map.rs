use crate::prelude::*;
use std::borrow::Borrow;

// mutable alias hashmap
#[derive(Serialize, Debug)]
pub struct CellMap<K, V>(RefCell<HashMap<K, V>>);

impl<K, V> CellMap<K, V> {
	pub fn get_mut(&mut self) -> &mut HashMap<K, V> {
		self.0.get_mut()
	}

	pub fn clear(&self) {
		self.0.borrow_mut().clear()
	}
}

impl<K, V> CellMap<K, V>
where
	V: Clone,
	K: Hash + Eq,
{
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&self, k: K, v: V) -> Option<V> {
		self.0.borrow_mut().insert(k, v)
	}

	pub fn get<Q>(&self, k: &K) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq,
	{
		self.0.borrow().get(k.borrow()).cloned()
	}

	pub fn remove<Q>(&self, k: &K) -> Option<V>
	where
		K: Borrow<Q>,
		Q: Hash + Eq,
	{
		self.0.borrow_mut().remove(k.borrow())
	}
}

impl<K, V> Default for CellMap<K, V> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<K, V> EguiInspect for CellMap<K, V>
where
	K: EguiInspect + fmt::Debug,
	V: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		ui.collapsing(label, |ui| {
			for (k, v) in self.0.borrow().iter() {
				v.inspect(&format!("{k:?}"), ui);
			}
		});
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		ui.collapsing(label, |ui| {
			if ui.button("❌ clear").clicked() {
				self.clear();
			}
			for (k, v) in self.0.borrow_mut().iter_mut() {
				v.inspect_mut(&format!("{k:?}"), ui);
			}
		});
	}
}

impl<'de, K, V> Deserialize<'de> for CellMap<K, V>
where
	K: Eq + Hash + Deserialize<'de>,
	V: Clone + Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		Ok(Self(RefCell::new(Deserialize::deserialize(deserializer)?)))
	}
}
