pub struct RndrReader;
use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};
use std::error::Error;
use tracing::error;

use directories::BaseDirs;

use crate::rndr_time::RndrTime;

pub type RndrPathResult = Result<String, Box<dyn Error>>;

const RNDR_LOG_EXTENSION: &str = "\\OtoyRndrNetwork\\rndr_log.txt";

impl RndrReader {
	///  gets the path to the rndr logs from appdata on windows.
	pub fn get_rndr_log_file() -> Result<String, Box<dyn Error>> {

		let base_dirs = BaseDirs::new().unwrap();
		let local_app_data_path = base_dirs.data_local_dir().to_str();

		match local_app_data_path {
			None => {
                error!("error finding local data path");
                Err("error finding local data path".into())
            },
			Some(datapath) => {
				let complete_appdata_path = format!(
					"{}{}",
					datapath.to_owned(),
					RNDR_LOG_EXTENSION
				);
				Ok(complete_appdata_path)
			}
		}
	}

	pub fn get_rndr_log_dir() -> RndrPathResult {
		let base_dirs = BaseDirs::new().unwrap();
		let local_app_data_path = base_dirs.data_local_dir().to_str();

		match local_app_data_path {
			None => Err("error finding local data path".into()),
			Some(datapath) => {
				let otoy_appdata_directory = format!(
					"{}\\OtoyRndrNetwork",
					datapath.to_owned()
				);
				Ok(otoy_appdata_directory)
			}
		}
	}

	/// creates a new BufReader (to reduce repetitive code writing for readers)
	pub fn new_log_reader() -> Result<BufReader<File>,Box<dyn Error>> {
		let log_file = RndrReader::get_rndr_log_file()
			.expect("error: rndr log could not be found.");

		let file = File::open(&log_file)?;
		let reader = BufReader::new(file);

		Ok(reader)
	}

	/// read_rndr_log - reads all the lines of the rndr log.
	pub fn get_latest_update() -> std::io::Result<()> {
		let reader = RndrReader::new_log_reader().expect("could not get log reader");

		let line_result =
            reader.lines().last().unwrap_or(Ok(" ".to_string()));
		let latest_line = line_result.unwrap_or(" ".to_string());
		RndrTime::check_new_event_update(latest_line);

		Ok(())
	}
	/// reads the rndr log once through
	pub fn read_rndr_log() -> Result<String, Box<dyn std::error::Error>> {
		let mut str_buffer = String::new();
		let mut reader = RndrReader::new_log_reader()?;
		reader.read_to_string(&mut str_buffer)
			.expect("could not read to string!");

		Ok(str_buffer)
	}
}
