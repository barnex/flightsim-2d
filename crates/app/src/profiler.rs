use crate::prelude::*;

static PROFILER: Mutex<Option<Profiler>> = Mutex::new(None);

pub fn start(label: &'static str) {
	PROFILER.lock().unwrap().get_or_insert_with(default).start(label)
}

pub fn stop(label: &'static str) {
	PROFILER.lock().unwrap().get_or_insert_with(default).stop(label)
}

pub fn restart(label: &'static str) {
	PROFILER.lock().unwrap().get_or_insert_with(default).restart(label)
}

pub fn scope<F, T>(label: &'static str, f: F) -> T
where
	F: FnOnce() -> T,
{
	start(label);
	let result = f();
	stop(label);
	result
}

pub fn to_string() -> String {
	PROFILER.lock().unwrap().get_or_insert_with(Profiler::new).to_string()
}

#[derive(Default)]
pub struct Profiler {
	timers: HashMap<&'static str, ProfTimer>,
}

#[derive(Default)]
struct ProfTimer {
	last_start_micros: Option<u64>,

	min: u64,
	max: u64,
	smooth: f64,
}

impl Profiler {
	fn new() -> Self {
		default()
	}

	fn start(&mut self, label: &'static str) {
		self.timers.entry(label).or_default().start();
	}

	fn stop(&mut self, label: &'static str) {
		self.timers.entry(label).or_default().stop();
	}

	fn restart(&mut self, label: &'static str) {
		self.timers.entry(label).or_default().restart();
	}
}

impl fmt::Display for Profiler {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let timers = &self.timers;
		for (label, timer) in timers.iter() {
			writeln!(f, "{label}: {timer}")?;
		}
		Ok(())
	}
}

impl fmt::Display for ProfTimer {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:0.03}ms ({:0.03} min, {:0.03} max)", self.smooth / 1000.0, self.min as f64 / 1000.0, self.max as f64 / 1000.0)?;
		Ok(())
	}
}

impl ProfTimer {
	fn restart(&mut self) {
		if self.last_start_micros.is_some() {
			self.stop();
		}
		self.start()
	}

	fn start(&mut self) {
		self.last_start_micros = Some(now())
	}

	fn stop(&mut self) {
		let Some(start) = self.last_start_micros.take() else {
			return log::error!("prof timer: stop: not started");
		};
		let elapsed = now().saturating_sub(start);
		self.max = u64::max(self.max, elapsed);
		self.min = if self.min == 0 { elapsed } else { u64::min(self.min, elapsed) };
		let a = 0.05;
		self.smooth = (1.0 - a) * self.smooth + a * (elapsed as f64);
	}
}

fn now() -> u64 {
	SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("system time").as_micros() as u64
}
