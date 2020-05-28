trait MyFormatting1 {
	/// 経過時間の文字列表現を得る
	fn to_string(&self) -> String;
	/// 経過時間の文字列表現を得る
	fn to_string2(&self) -> String;
}

impl MyFormatting1 for std::time::Duration {
	fn to_string(&self) -> String {
		let mut sec = self.as_secs();
		let mut min = 0;
		let mut hour = 0;
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}
		let s = format!("{:02}:{:02}:{:02}", hour, min, sec);
		return s;
	}

	fn to_string2(&self) -> String {
		let mut millis = self.as_millis();
		let mut sec = 0;
		let mut min = 0;
		let mut hour = 0;
		while 1000 <= millis {
			sec += 1;
			millis -= 1000;
		}
		while 60 <= sec {
			min += 1;
			sec -= 60;
		}
		while 60 <= min {
			hour += 1;
			min -= 60;
		}
		let s = format!("{:02}:{:02}:{:02}:{:03}", hour, min, sec, millis);
		return s;
	}
}

pub struct Stopwatch {
	_time: std::time::Instant,
}

impl Stopwatch {
	/// オブジェクトを生成します。
	pub fn new() -> Stopwatch {
		return Stopwatch { _time: std::time::Instant::now() };
	}

	/// 経過時間の文字列表現を返します。
	pub fn elapsed_text(self: &Stopwatch) -> String {
		let elapsed = std::time::Instant::now() - self._time;
		return format!("{}", elapsed.to_string2());
	}
}

#[allow(dead_code)]
fn format_duration(left: std::time::Instant, right: std::time::Instant) -> String {
	let elapsed = right - left;
	return elapsed.to_string2();
}
