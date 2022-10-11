extern crate directories;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{sync::mpsc::channel, time::Duration};

pub mod rndr_reader;
pub mod rndr_time;
mod stats;
mod web_api;
use stats::RndrStats;


use tracing::{info, error, warn};

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

    // start our tracing subscriber.
    tracing_subscriber::fmt::init();

	let watch_duration = Duration::from_secs(4);
	let registry_data = RndrStats::get_registry_data();
	let rndrlog_watch_path = RndrReader::get_rndr_log_dir().unwrap();

	let mut watcher = watcher(sx, watch_duration).unwrap();
	watcher
		.watch(rndrlog_watch_path.to_string(), RecursiveMode::NonRecursive)
		.expect(
		"could not watch the file, something very wrong has happened.",
	);

	// when the program starts for the firs time, print the whole files info as a string.
	// IDEA here. Should this be passed up to the user for clarification,
	// or for link access?
	info!("{}", rndrlog_watch_path);
	info!("{:#?}", registry_data); // NOTE THIS SHOULD NOT BE PRINTED, ONLY PASS AN OK
	info!("this is the log so far: \n {}", RndrReader::read_rndr_log());

	// NOTE this goes to the frontend through the app API,
	// but probably good to be logging it as well.
	info!(
		"minutes spent rendering: {:?}",
		RndrTime::total_time_in_minutes()
	);

	loop {
		match rx.recv() {
            /*
			* Send:
			* latest update RndrLog
			* total jobs completed
			*/
			Ok(DebouncedEvent::Write(event)) => {
				RndrReader::get_latest_update().unwrap(); 	// NOTE this goes to the frontend of the app.
				info!("Write Event: {:#?}", event); 		// NOTE this can probably be removed, not sure if the end user want to see the event type.
				info!(
					"minutes spent rendering: {:?}\n",
					RndrTime::total_time_in_minutes()
				);
                println!("jobs completed: {}", registry_data.jobs_completed);
			}
            /*
			 * Send:
			 * RndrLog latest update
			 * total jobs completed.
			 */
			Ok(DebouncedEvent::Create(e)) => {
				let path_buf = e.as_path();
				info!("Create event called!: {:#?}", path_buf);


				RndrReader::get_latest_update().unwrap();	// NOTE this goes to the frontend of the app.
				info!(
					"minutes spent rendering: {:?}\n",
					RndrTime::total_time_in_minutes()
				);
			}
			/*
			* Covers case of DebounceEvent errors,
			* NOT a match error.
			*/
			Ok(DebouncedEvent::Error(why, _)) => {
				error!("{why:?}");
			}
			/*
			* UNKNOWN DebounceEvent case, just in case it is useful.
			* This may be taken out later, so keep this comment for reference.
			*/
			Ok(event) => println!(
				"event occured that is not covered in this scope.{:?}",
				event
			),

			Err(why) => {
				error!("error occurred: {:?}", why);
			}
		}
	}
}
