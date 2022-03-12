pub struct RndrReader;
use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};

use directories::BaseDirs;

use crate::rndr_time::RndrTime;

pub type RndrPathResult = Result<String, String>;

const RNDR_LOG_EXTENSION: &str = "\\OtoyRndrNetwork\\rndr_log.txt";

impl RndrReader {
	// get_rndr_log_path - gets the path to the rndr logs from appdata on windows.
	pub fn get_rndr_log_path() -> RndrPathResult {
		let base_dirs = BaseDirs::new().unwrap();
		let local_app_data_path = base_dirs.data_local_dir().to_str();

		match local_app_data_path {
			None => Err("error finding local data path".to_owned()),
			Some(datapath) => {
				let complete_appdata_path =
					format!("{}{}", datapath.to_owned(), RNDR_LOG_EXTENSION);
				Ok(complete_appdata_path)
			}
		}
	}

	// new_log_reader - creates a new BufReader (to reduce repetitive code writing for readers)
	pub fn new_log_reader() -> BufReader<File> {
		let log_file = RndrReader::get_rndr_log_path()
			.expect("error: rndr log could not be found.");
		let file = File::open(log_file).expect("error: file not found!");
		let reader = BufReader::new(file);
		reader
	}

	/// read_rndr_log - reads all the lines of the rndr log.
	/// Returns a union Result.
	pub fn get_latest_update() -> std::io::Result<()> {
		let mut str_buffer = String::new();
		let mut reader = RndrReader::new_log_reader();
		reader.read_to_string(&mut str_buffer).unwrap();
		let latest_line = reader.lines().last().unwrap();
		RndrTime::check_new_event_update(latest_line?);
		Ok(())
	}
	pub fn read_rndr_log() -> String {
		let mut str_buffer = String::new();
		let mut reader = RndrReader::new_log_reader();
		reader.read_to_string(&mut str_buffer).unwrap();
		str_buffer
	}
}
