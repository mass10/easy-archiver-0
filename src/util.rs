//!
//! ユーティリティー
//!

/// タイムスタンプ "%Y-%m-%d %H:%M:%S%.3f" を返します。
#[allow(unused)]
pub fn timestamp0() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
}

/// タイムスタンプ "%Y%m%d-%H%M%S" を返します。
#[allow(unused)]
pub fn timestamp1() -> String {
	let date = chrono::Local::now();
	return format!("{}", date.format("%Y%m%d-%H%M%S"));
}
