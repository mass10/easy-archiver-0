//!
//! 日付、および時刻関連の汎用操作を提供しています。
//!

use crate::helpers;

/// ストップウォッチです。
pub struct Stopwatch {
	/// インスタンスが生成された、もしくはオブジェクトがリセットされた日時を指します。
	_time: std::time::Instant,
}

impl Stopwatch {
	/// オブジェクトを生成します。
	pub fn new() -> Stopwatch {
		return Stopwatch { _time: std::time::Instant::now() };
	}
}

impl std::fmt::Display for Stopwatch {
	/// 経過時間の文字列表現を返します。
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		use helpers::MyDurationHelper;
		let elapsed = std::time::Instant::now() - self._time;
		write!(f, "{}", elapsed.to_string2())?;
		return Ok(());
	}
}
