type TimerCallback = fn();

pub struct Timer {
	pub value: u8,
	is_active: bool,
	timer_callback: TimerCallback
}

impl Timer {
	pub fn new(timer_callback: TimerCallback) -> Timer {
		Timer {
			value: 0,
			is_active: false,
			timer_callback
		}
	}

	pub fn new_no_callback() -> Timer {
		Timer {
			value: 0,
			is_active: false,
			timer_callback: Timer::do_nothing
		}
	}

	fn do_nothing() {}

	pub fn tick(&mut self) {
		if self.is_active {
			self.value -= 1;
		}

		if self.value > 0 && !self.is_active {
			self.is_active = true;
			self.value -= 1;
			let callback = self.timer_callback;
			callback();
		}

		if self.value == 0 {
			self.is_active = false;
		}
	}
}