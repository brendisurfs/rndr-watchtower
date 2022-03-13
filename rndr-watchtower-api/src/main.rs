extern crate directories;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};

pub mod rndr_reader;
pub mod rndr_time;
mod stats;
use stats::RndrStats;

use crate::{rndr_reader::RndrReader, rndr_time::RndrTime};

#[derive(Debug, Clone, PartialEq, Default)]
struct RndrLog {
	date: String,
	time: String,
	status_code: String,
	msg: String,
}

fn main() {
	let (sx, rx) = channel();

	let watch_duration = Duration::from_secs(4);
	let watch_path = RndrReader::get_rndr_log_dir().unwrap();
	// let watch_log = RndrReader::get_rndr_log_file().unwrap();

	let mut watcher = watcher(sx, watch_duration).unwrap();

	watcher
		.watch(watch_path.to_string(), RecursiveMode::NonRecursive)
		.expect(
		"could not watch the file, something very wrong has happened.",
	);

	let registry_data = RndrStats::get_registry_data();

	// when the program starts for the firs time, print the whole files info as a string.
	println!("{:#?}", registry_data);
	println!("this is the log so far: \n {}", RndrReader::read_rndr_log());
	println!(
		"minutes spent rendering: {:?}",
		RndrTime::total_time_in_minutes()
	);

	loop {
		match rx.recv() {
			Ok(DebouncedEvent::Write(event)) => {
				println!("Write Event: {:#?}", event);
				RndrReader::get_latest_update().unwrap();
				println!(
					"minutes spent rendering: {:?}\n",
					RndrTime::total_time_in_minutes()
				);
			}
			Ok(DebouncedEvent::Create(e)) => {
				let path_buf = e.as_path();
				println!("Create event called!: {:#?}", path_buf);
				RndrReader::get_latest_update().unwrap();
				println!(
					"minutes spent rendering: {:?}\n",
					RndrTime::total_time_in_minutes()
				);
			}
			// incase it errors
			Ok(DebouncedEvent::Error(e, _)) => {
				println!("{e}");
			}
			Err(e) => {
				eprintln!("error occurred: {:?}", e);
			}
			_ => println!(
				"event occured that is not covered in this scope.",
			),
		}
	}
}
