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
	let (sender, receiver) = channel();

	// // watches our rndr logs for any changes.
	let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();
	let watch_path = RndrReader::get_rndr_log_path().unwrap();
	watcher
		.watch(watch_path, RecursiveMode::NonRecursive)
		.unwrap();

	RndrStats::new_data();
	println!("this is the log so far: \n {}", RndrReader::read_rndr_log());
	println!("minutes spent rendering {:?}", RndrTime::time_in_minutes());

	// watch for file changes here
	loop {
		match receiver.recv() {
			Ok(DebouncedEvent::Write(_)) => {
				RndrReader::get_latest_update().unwrap();
				println!(
					"{:?} minutes spent rendering",
					RndrTime::time_in_minutes()
				);
			}
			Err(e) => println!("watch error: {:?}", e),
			_ => (),
		}
	}
}
