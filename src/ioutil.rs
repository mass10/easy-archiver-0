/// 標準入力から1行読み込みます。終端の改行文字を除く1行全体を返します。
#[allow(unused)]
pub fn read_line() -> String {
	let mut line = String::new();
	let result = std::io::stdin().read_line(&mut line);
	if result.is_err() {
		return String::new();
	}
	return line.trim().to_string();
}

/// エンターキーが押されるまで待機します。
#[allow(unused)]
pub fn pause() {
	let _ = read_line();
}

/// Yes/No で回答すべきプロンプトを表示します。
#[allow(unused)]
pub fn prompt(message: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
	use std::io::Write; // -> Stdout に flush() を実装するトレイト

	println!("{}", message);
	loop {
		print!("(y/N): ");
		std::io::stdout().flush().unwrap();
		let answer = read_line().to_uppercase();
		if answer == "Y" || answer == "YES" {
			return Ok(true);
		}
		if answer == "N" || answer == "NO" {
			return Ok(false);
		}
	}
}
