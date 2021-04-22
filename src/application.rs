extern crate chrono;
extern crate regex;

/// ディレクトリの妥当性を検証します。
fn is_valid_directory(dir: &std::path::Path) -> bool {
	match dir.file_name().unwrap().to_str().unwrap() {
		"node_modules" => return false,
		".git" => return false,
		"dist" => return false,
		".nuxt" => return false,
		"Debug" => return false,
		"Release" => return false,
		"ReleaseDebug" => return false,
		"target" => return false,
		"ipch" => return false,
		"x64" => return false,
		_ => {}
	};
	return true;
}

#[allow(unused)]
fn matches(regex: &str, text: &str) -> bool {
	let reg = regex::Regex::new(regex);
	if reg.is_err() {
		panic!("[ERROR] 正規表現がエラー (理由: {})", reg.err().unwrap());
	}
	let result = reg.unwrap().find(text);
	if result.is_none() {
		return false;
	}
	return true;
}

/// ファイル名の検証
fn is_valid_file(dir: &std::path::Path) -> bool {
	#[allow(unused)]
	let name = dir.file_name().unwrap().to_str().unwrap();
	// で終わる
	if matches("-20[0-9][0-9][0-9][0-9][0-9][0-9]-[0-9][0-9][0-9][0-9].zip$", name) {
		return false;
	}
	// で終わる
	if matches("-20[0-9][0-9][0-9][0-9][0-9][0-9]-[0-9][0-9][0-9][0-9][0-9][0-9].zip$", name) {
		return false;
	}
	// で終わる
	if matches(".VC.db$", name) {
		return false;
	}
	// で終わる
	if matches(".ipch$", name) {
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
	if source_path.is_dir() {
		// ディレクトリ
		// ディレクトリの妥当性を検証します。
		if !is_valid_directory(source_path) {
			return Ok(0);
		}
		// コピー先にディレクトリを作成します。
		std::fs::create_dir_all(destination_path)?;
		// ディレクトリ内エントリーを走査
		let mut files_affected = 0;
		let it = std::fs::read_dir(source_path)?;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			let path = entry.path();
			files_affected += xcopy(&path.to_str().unwrap(), destination_path.join(name).as_path().to_str().unwrap())?;
		}
		return Ok(files_affected);
	} else if source_path.is_file() {
		// ファイル
		if !is_valid_file(source_path) {
			return Ok(0);
		}
		println!("(+) {}", destination_path.as_os_str().to_str().unwrap());
		std::fs::copy(source_path, destination_path)?;
		std::thread::sleep(std::time::Duration::from_micros(1));
		return Ok(1);
	} else {
		// 不明なファイルシステム
		println!("[WARN] 不明なファイルシステムです。{}", source_path.as_os_str().to_str().unwrap());
		return Ok(0);
	}
}

/// フルパスに変換
fn get_absolute_path(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	let absolute_path = std::fs::canonicalize(path)?;
	let result = absolute_path.as_path().as_os_str().to_str().unwrap();
	return Ok(result.to_string());
}

/// 書庫化 & 圧縮
fn compress(path_to_directory: &str, zip_archive_name: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	let command_path = "C:\\Program Files\\7-Zip\\7z.exe";
	let mut command = std::process::Command::new(command_path);
	let args = ["a", zip_archive_name, path_to_directory];
	// 7zip 呼び出し
	let mut command = command.args(&args).spawn()?;
	let status = command.wait()?;
	if !status.success() {
		let exit_code = status.code().unwrap();
		println!("[WARN] yarn exited with status: {}", exit_code);
		std::process::exit(exit_code);
	}
	return Ok(());
}

/// 一時ディレクトリ
fn create_temp_dir(path: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
	use crate::util;

	// タイムスタンプ(%Y%m%d-%H%M%S)
	let current_timestamp = util::timestamp1();
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
pub fn zip(path_to_target: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
	use super::util;

	// フルパスに変換
	let absolute_path = get_absolute_path(path_to_target)?;
	// タイムスタンプ(%Y%m%d-%H%M%S)
	let current_timestamp = util::timestamp1();
	// 一時ディレクトリ
	let tmp_dir = create_temp_dir(&absolute_path)?;
	// バックアップ対象ファイルを列挙します。
	let files_affected = xcopy(&absolute_path, &tmp_dir)?;
	// 新しいパス
	let zip_archive_name = format!("{}-{}.zip", absolute_path, current_timestamp);
	println!("[TRACE] destination: {}", zip_archive_name.as_str());

	// 書庫化
	compress(&tmp_dir, zip_archive_name.as_str())?;

	println!("[TRACE] {}個のファイルをコピーしました。", files_affected);

	return Ok(());
}
