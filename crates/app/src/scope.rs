use crate::prelude::*;

static SCOPE: Mutex<Option<Scope>> = Mutex::new(None);

pub fn record<T: fmt::Debug>(label: &'static str, value: T) {
	let value = format!("{value:?}");
	SCOPE.lock().unwrap().get_or_insert_with(default).record(label, value)
}

pub fn to_string() -> String {
	SCOPE.lock().unwrap().get_or_insert_with(Scope::new).to_string()
}

#[derive(Default)]
pub struct Scope {
	values: HashMap<&'static str, String>,
}

impl Scope {
	fn new() -> Self {
		default()
	}

	fn record(&mut self, label: &'static str, value: String) {
		self.values.insert(label, value);
	}
}

impl fmt::Display for Scope {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for (label, value) in &self.values {
			writeln!(f, "{label}: {value}")?;
		}
		Ok(())
	}
}
