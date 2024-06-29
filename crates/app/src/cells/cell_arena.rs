use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct CellArena<T> {
	slots: Vec<Slot<T>>,
	push_list: RefCell<Vec<(usize, Slot<T>)>>,
	delete_list: RefCell<Vec<(u32, u32)>>,
	free: RefCell<Vec<u32>>,
	next_generation: Cell<u32>,
	next_index: Cell<u32>,
}

pub trait HasID: Sized {
	fn id_mut(&mut self) -> &mut ID<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
struct Slot<T> {
	value: T,
	generation: Cell<u32>,
}

impl<T> CellArena<T> {
	// Must be called each frame
	pub fn gc(&mut self) {
		// apply push list
		for (index, slot) in self.push_list.get_mut().drain(..) {
			if index < self.slots.len() {
				debug_assert!(self.slots[index].generation.get() == GEN_DELETED);
				self.slots[index] = slot;
			} else {
				assert!(index == self.slots.len());
				self.slots.push(slot)
			}
		}
		// THEN apply delete list
		//for (index, generation) in self.delete_list.get_mut().drain(..) {
		//	todo!("delete")
		//}
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
		// TODO: omit deleted
		self.slots.iter_mut().map(|slot| &mut slot.value)
	}

	pub fn iter(&self) -> impl Iterator<Item = &'_ T> {
		self //_
			.slots
			.iter()
			.filter(|slot| slot.generation.get() != GEN_DELETED)
			.map(|slot| &slot.value)
	}

	pub fn enumerate(&self) -> impl Iterator<Item = (ID<T>, &'_ T)> {
		// TODO: omit deleted
		self.slots.iter().enumerate().map(|(i, slot)| (ID::new(i, slot.generation.get()), &slot.value))
	}

	pub fn ids(&self) -> impl Iterator<Item = ID<T>> + '_ {
		self.slots.iter().enumerate().map(|(i, slot)| ID::new(i, slot.generation.get()))
	}

	pub fn get(&self, id: ID<T>) -> Option<&T> {
		self.slots //_
			.get(id.index as usize)
			.filter(|slot| (slot.generation.get() == id.generation))
			.map(|slot| &slot.value)
	}

	pub fn get_mut(&mut self, id: ID<T>) -> Option<&mut T> {
		self.slots //_
			.get_mut(id.index as usize)
			.filter(|slot| (slot.generation.get() == id.generation))
			.map(|slot| &mut slot.value)
	}

	pub fn len(&self) -> usize {
		self.slots.len() - self.free.borrow().len()
	}

	fn next_index(&self) -> usize {
		let index = self.next_index.get();
		self.next_index.set(index + 1);
		index as usize
	}

	pub fn remove_now(&mut self, id: ID<T>) {
		debug_assert!((id.index as usize) < self.slots.len());
		if self.slots[id.index as usize].generation.get() != GEN_DELETED {
			self.free.borrow_mut().push(id.index);
			self.slots[id.index as usize].generation.set(GEN_DELETED);
		}
	}

	pub fn next_generation(&self) -> u32 {
		let mut curr = self.next_generation.get();
		curr = curr.wrapping_add(1);
		if curr == GEN_DELETED {
			curr += 1;
		}
		self.next_generation.set(curr);
		curr
	}

	//pub fn defer_remove(&self, index: usize) {
	//	//self.remove_list.borrow_mut().insert(index);
	//	//let mut push_list = self.push_list.borrow_mut();
	//	//let index = self.inner.len() + push_list.len();
	//	//push_list.push(item);
	//	//index
	//}
}

const GEN_DELETED: u32 = 0;

impl<T> CellArena<T>
where
	T: HasID,
{
	// pub fn clear(&self) { }

	pub fn push_now(&mut self, mut value: T) -> ID<T> {
		let generation = self.next_generation();

		let index = match self.free.get_mut().pop() {
			Some(index) => {
				let index = index as usize;
				debug_assert!(index < self.slots.len());
				debug_assert!(self.slots[index].generation.get() == 0);
				index
			}
			None => self.next_index(),
		};

		let id = ID::new(index, generation);

		*value.id_mut() = id;

		let slot = Slot {
			value,
			generation: Cell::new(generation),
		};

		if index < self.slots.len() {
			self.slots[index] = slot;
		} else {
			self.slots.push(slot);
		}

		id
	}

	pub fn defer_push(&self, mut value: T) -> ID<T> {
		let generation = self.next_generation();

		let index = match self.free.borrow_mut().pop() {
			Some(index) => {
				let index = index as usize;
				debug_assert!(index < self.slots.len());
				debug_assert!(self.slots[index].generation.get() == 0);
				index
			}
			None => self.next_index(),
		};
		let id = ID::new(index, generation);
		*value.id_mut() = id;

		let slot = Slot {
			value,
			generation: Cell::new(generation),
		};
		self.push_list.borrow_mut().push((index, slot));
		id
	}
}

impl<A> FromIterator<A> for CellArena<A>
where
	A: HasID,
{
	fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
		let mut temp = Self::default();
		for item in iter {
			temp.push_now(item);
		}
		temp
	}
}

// old-style
impl<T> std::ops::Index<ID<T>> for CellArena<T> {
	type Output = T;

	fn index(&self, index: ID<T>) -> &Self::Output {
		self.get(index).expect("invalid index")
	}
}

// old-style
impl<T> std::ops::IndexMut<ID<T>> for CellArena<T> {
	fn index_mut(&mut self, index: ID<T>) -> &mut Self::Output {
		self.get_mut(index).expect("invalid index")
	}
}

impl<T> Default for CellArena<T> {
	fn default() -> Self {
		Self {
			slots: default(),
			push_list: default(),
			free: default(),
			next_generation: default(),
			next_index: default(),
			delete_list: default(),
		}
	}
}

impl<T> fmt::Debug for CellArena<T>
where
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ArenaCell")
			.field("slots", &self.slots)
			.field("push_list", &self.push_list)
			.field("free", &self.free)
			.field("next_generation", &self.next_generation)
			.field("next_index", &self.next_index)
			.finish()
	}
}

impl<T> EguiInspect for CellArena<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.slots.inspect(label, ui)
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.slots.inspect_mut(label, ui)
	}
}

impl<T> EguiInspect for Slot<T>
where
	T: EguiInspect,
{
	fn inspect(&self, label: &str, ui: &mut egui::Ui) {
		self.value.inspect(label, ui)
	}
	fn inspect_mut(&mut self, label: &str, ui: &mut egui::Ui) {
		self.value.inspect_mut(label, ui)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	/*
	#[test]
	fn mutable() {
		let mut a = CellArena::<(ID<String>, String)>::default();

		assert_eq!(a.len(), 0);

		dbg!(&a);

		let ifoo = dbg!(a.push_now("foo".into()));
		dbg!(&a);

		let ibar = dbg!(a.push_now("bar".into()));
		let ibaz = dbg!(a.push_now("baz".into()));
		dbg!(&a);

		assert_eq!(a.len(), 3);

		assert_eq!(a.get(ifoo), Some(&"foo".into()));
		assert_eq!(a.get(ibar), Some(&"bar".into()));
		assert_eq!(a.get(ibaz), Some(&"baz".into()));

		dbg!(a.remove_now(ibar));
		dbg!(&a);
		assert_eq!(a.len(), 2);

		assert_eq!(a.get(ifoo), Some(&"foo".into()));
		assert_eq!(a.get(ibar), None);
		assert_eq!(a.get(ibaz), Some(&"baz".into()));

		let icat = dbg!(a.push_now("cat".into()));
		dbg!(&a);
		assert_eq!(a.get(ifoo), Some(&"foo".into()));
		assert_eq!(a.get(ibar), None);
		assert_eq!(a.get(ibaz), Some(&"baz".into()));
		assert_eq!(a.get(icat), Some(&"cat".into()));
	}

	#[test]
	fn shared() {
		let mut a = CellArena::<String>::default();

		let (ifoo, ibar, ibaz);

		{
			let a = &a;

			dbg!(a);
			assert_eq!(a.len(), 0);

			ifoo = dbg!(a.defer_push("foo".into()));
			ibar = dbg!(a.defer_push("bar".into()));
			ibaz = dbg!(a.defer_push("baz".into()));

			dbg!(a);
			assert_eq!(a.len(), 0);

			assert_eq!(a.get(ifoo), None);
			assert_eq!(a.get(ibar), None);
			assert_eq!(a.get(ibaz), None);
		}

		println!("gc...");
		a.gc();

		{
			let a = &a;
			dbg!(a);
			assert_eq!(a.len(), 3);
			assert_eq!(a.get(ifoo), Some(&"foo".into()));
			assert_eq!(a.get(ibar), Some(&"bar".into()));
			assert_eq!(a.get(ibaz), Some(&"baz".into()));
		}

		//{
		//	a.remove_now(ibar);
		//	assert_eq!(a.len(), 2);

		//	assert_eq!(a.get_id(ifoo), Some(&"foo".into()));
		//	assert_eq!(a.get_id(ibar), None);
		//	assert_eq!(a.get_id(ibaz), Some(&"baz".into()));

		//	let icat = a.push_now("cat".into());
		//	assert_eq!(a.get_id(ifoo), Some(&"foo".into()));
		//	assert_eq!(a.get_id(ibar), None);
		//	assert_eq!(a.get_id(ibaz), Some(&"baz".into()));
		//	assert_eq!(a.get_id(icat), Some(&"cat".into()));
		//}
	}
	*/
}
