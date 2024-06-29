use crate::*;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Stats {
	pub frame: [Mut<u64>; Event::_Num as usize],
	pub total: [Mut<u64>; Event::_Num as usize],
}

impl Stats {
	pub fn iter(&self) -> impl Iterator<Item = (Event, u64, u64)> + '_ {
		Event::all().map(|stat| (stat, self.frame[stat as usize].get(), self.total[stat as usize].get()))
	}
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Event {
	CallAnimateCrab = 0,
	CallNavigateCrab = 1,
	NavigationStuck = 2,
	CallRelax = 3,
	GiveWorkOk = 4,
	GiveWorkErr = 5,
	PickUpOk = 6,
	PickUpErr = 7,
	DropOffOk = 8,
	DropOffErr = 9,
	EmptyHands = 10,
	AvailableAssignedCrabs = 11,
	HarvestTilesScanned = 12,
	HarvestResourcesFound = 13,
	_Num = 14, // used to determine array size. Must be last.
}

impl Event {
	pub fn all() -> impl Iterator<Item = Event> {
		(0..(Event::_Num as usize)).map(|i| Event::from_usize(i).unwrap())
	}
}

impl Stats {
	pub fn inc(&self, stat: Event) {
		self.add(stat, 1)
	}

	pub fn add(&self, stat: Event, n: u64) {
		self.frame[stat as usize].increment(n);
	}

	/// Must be called at the beginning of a frame.
	pub fn start_frame(&self) {
		for (frame, total) in iter::zip(&self.frame, &self.total) {
			total.increment(frame.get());
			frame.set(0)
		}
	}
}

impl fmt::Display for Stats {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for i in 0..(Event::_Num as usize) {
			let stat = Event::from_u8(i as u8).expect("Stats: bug");
			writeln!(f, "{:?}: {}, {}", stat, self.frame[i], self.total[i])?;
		}
		Ok(())
	}
}
