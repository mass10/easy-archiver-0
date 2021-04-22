//!
//! エントリーポイントの定義
//!

extern crate chrono;
extern crate regex;

mod application;
mod helpers;
mod timeutil;
mod util;

/// エントリーポイントです。
fn main() {
	// コマンドライン引数(コマンド自身を除く)
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("パスを指定します。");
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// 処理時間計測用ストップウォッチ
	let stopwatch = timeutil::Stopwatch::new();

	// 第一引数
	let path_to_target = &args[0];

	// 書庫化 & ZIP 圧縮
	let result = application::zip(&path_to_target);
	if result.is_err() {
		println!("[ERROR] エラー！理由: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// サマリー
	println!("[TRACE] end. (処理時間: {})", stopwatch);

	std::thread::sleep(std::time::Duration::from_secs(2));
}
