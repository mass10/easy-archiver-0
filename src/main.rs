extern crate chrono;

/// ユーティリティー
struct Util {}

impl Util {
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
}

/// 標準入力から1行読み込みます。終端の改行文字を除く1行全体を返します。
#[allow(unused)]
fn read_line() -> String {
	let mut line = String::new();
	let result = std::io::stdin().read_line(&mut line);
	if result.is_err() {
		return String::new();
	}
	return line.trim().to_string();
}

/// エンターキーが押されるまで待機します。
#[allow(unused)]
fn pause() {
	let _ = read_line();
}

/// Yes/No で回答すべきプロンプトを表示します。
#[allow(unused)]
fn prompt(message: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
	use std::io::Write;

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

/// std::time::Duration にアプリケーション固有のフォーマット操作を定義しています。
trait MyDurationHelper {
	/// 経過時間の文字列表現を得る
	fn to_string1(&self) -> String;
	/// 経過時間の文字列表現を得る
	fn to_string2(&self) -> String;
}

impl MyDurationHelper for std::time::Duration {
	fn to_string1(&self) -> String {
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

/// ストップウォッチです。
struct Stopwatch {
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
		let elapsed = std::time::Instant::now() - self._time;
		write!(f, "{}", elapsed.to_string2())?;
		return Ok(());
	}
}

/// ディレクトリの妥当性を検証します。
fn is_valid_directory(dir: &std::path::Path) -> bool {
	let dir_name = dir.file_name().unwrap().to_str().unwrap();
	if dir_name == "node_modules" {
		return false;
	}
	if dir_name == ".git" {
		return false;
	}
	if dir_name == "dist" {
		return false;
	}
	if dir_name == ".nuxt" {
		return false;
	}
	if dir_name == "Debug" {
		return false;
	}
	if dir_name == "Release" {
		return false;
	}
	if dir_name == "ReleaseDebug" {
		return false;
	}
	if dir_name == "target" {
		return false;
	}
	return true;
}

/// ディレクトリをコピーします。
fn xcopy(left: &str, right: &str) -> std::result::Result<u32, Box<dyn std::error::Error>> {
	// 元
	let source_path = std::path::Path::new(left);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", left);
		return Ok(0);
	}
	// 先
	let destination_path = std::path::Path::new(right);
	// ディレクトリ
	if source_path.is_dir() {
		// ディレクトリの妥当性を検証します。
		if !is_valid_directory(source_path) {
			return Ok(0);
		}
		// コピー先にディレクトリを作成します。
		std::fs::create_dir_all(destination_path)?;
		// ディレクトリ内エントリーを走査
		let mut files_copied = 0;
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			let path = entry.path();
			files_copied += xcopy(&path.to_str().unwrap(), destination_path.join(name).as_path().to_str().unwrap())?;
		}
		return Ok(files_copied);
	}
	// ファイル
	if source_path.is_file() {
		println!("(+) {}", destination_path.as_os_str().to_str().unwrap());
		std::fs::copy(source_path, destination_path)?;
		std::thread::sleep(std::time::Duration::from_millis(1));
		return Ok(1);
	}
	println!("[WARN] 不明なファイルシステムです。{}", source_path.as_os_str().to_str().unwrap());
	return Ok(0);
}

/// フルパスに変換
fn get_absolute_path(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let absolute_path = std::fs::canonicalize(path)?;
	let result = absolute_path.as_path().as_os_str().to_str().unwrap();
	return Ok(result.to_string());
}

/// 書庫化 & 圧縮(ZIP)
fn compress(path_to_directory: &str, zip_archive_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// 7zip 呼び出し
	let command_path = "C:\\Program Files\\7-Zip\\7z.exe";
	let mut command = std::process::Command::new(command_path);
	let args = ["a", zip_archive_name, path_to_directory];
	let mut command = command.args(&args).spawn()?;
	let status = command.wait()?;

	// 終了ステータスの確認
	if !status.success() {
		// バッチを終了
		let exit_code = status.code().unwrap();
		println!("[WARN] yarn exited with status: {}", exit_code);
		std::process::exit(exit_code);
	}

	return Ok(());
}

/// 一時ディレクトリ
fn create_temp_dir(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	// タイムスタンプ(%Y%m%d-%H%M%S)
	let current_timestamp = Util::timestamp1();

	// ディレクトリ名
	let name = get_name_only(&path)?;

	// 一時ディレクトリの下に同名のフォルダーを作る
	let tmp_dir = format!("C:\\tmp\\.{}.tmp", current_timestamp);
	let tmp_dir = std::path::Path::new(&tmp_dir).join(name);
	let tmp_dir = tmp_dir.to_str().unwrap();

	// ディレクトリを作成
	std::fs::create_dir_all(tmp_dir)?;

	return Ok(tmp_dir.to_string());
}

/// ディレクトリ／ファイルの名前だけを取り出します。
fn get_name_only(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let name = std::path::Path::new(path).file_name().unwrap().to_str().unwrap();
	return Ok(name.to_string());
}

/// 書庫化 & ZIP 圧縮
fn zip(path_to_target: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	// フルパスに変換
	let absolute_path = get_absolute_path(path_to_target)?;
	// タイムスタンプ(%Y%m%d-%H%M%S)
	let current_timestamp = Util::timestamp1();
	// 一時ディレクトリ
	let tmp_dir = create_temp_dir(&absolute_path)?;
	// バックアップ対象ファイルを列挙します。
	let files_copied = xcopy(&absolute_path, &tmp_dir)?;
	// 新しいパス
	let zip_archive_name = format!("{}-{}.zip", absolute_path, current_timestamp);
	println!("[TRACE] destination: {}", zip_archive_name.as_str());

	// 書庫化
	compress(&tmp_dir, zip_archive_name.as_str())?;

	println!("[TRACE] {}個のファイルをコピーしました。", files_copied);

	return Ok(());
}

/// エントリーポイントです。
fn main() {
	// コマンドライン引数(コマンド自身を除く)
	let args: std::vec::Vec<String> = std::env::args().skip(1).collect();
	if args.len() == 0 {
		println!("パスを指定します。");
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// 第一引数のみ
	let path_to_target = &args[0];

	// 処理時間計測用ストップウォッチ
	let stopwatch = Stopwatch::new();

	// 書庫化 & ZIP 圧縮
	let result = zip(&path_to_target);
	if result.is_err() {
		println!("[ERROR] エラー！理由: {:?}", result.err().unwrap());
		std::thread::sleep(std::time::Duration::from_secs(2));
		return;
	}

	// サマリー
	println!("[TRACE] end. (処理時間: {})", stopwatch);

	std::thread::sleep(std::time::Duration::from_secs(2));
}
