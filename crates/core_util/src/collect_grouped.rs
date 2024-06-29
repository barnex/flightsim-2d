use std::collections::HashMap; // << TODO: allow different hasher
use std::hash::Hash;

pub trait CollectGrouped {
	type Key;
	type Value;
	fn collect_grouped(self) -> HashMap<Self::Key, Vec<Self::Value>>;
}

impl<I, K, V> CollectGrouped for I
where
	I: Iterator<Item = (K, V)>,
	K: Eq + Hash,
{
	type Key = K;
	type Value = V;

	fn collect_grouped(self) -> HashMap<Self::Key, Vec<Self::Value>> {
		let mut result = HashMap::<K, Vec<V>>::default();
		for (k, v) in self {
			result.entry(k).or_default().push(v);
		}
		result
	}
}
