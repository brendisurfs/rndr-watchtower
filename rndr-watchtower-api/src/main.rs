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

	let watch_duration = Duration::from_millis(10);
	let watch_path = RndrReader::get_rndr_log_path().unwrap();

	let mut watcher = watcher(sx, watch_duration).unwrap();

	watcher
		.watch(watch_path, RecursiveMode::NonRecursive)
		.unwrap();

	let registry_data = RndrStats::get_registry_data();

	// when the program starts for the firs time, print the whole files info as a string.
	println!("{:#?}", registry_data);
	println!("this is the log so far: \n {}", RndrReader::read_rndr_log());
	println!("minutes spent rendering {:?}", RndrTime::time_in_minutes());

	loop {
		match rx.recv() {
			Ok(DebouncedEvent::Write(_)) => {
				RndrReader::get_latest_update().unwrap();
				println!(
					"{:?} minutes spent rendering",
					RndrTime::time_in_minutes()
				);
			}
			// incase it errors
			Ok(DebouncedEvent::Error(e, _)) => {
				println!("{e}");
			}
			_ => print!("unhandled event occured, does not fit match criteria"),
		}
	}
}
