extern crate directories;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use tokio::signal;
use std::{sync::mpsc::channel, time::Duration};

pub mod rndr_reader;
pub mod rndr_time;
mod stats;
mod web_api;
use stats::RndrStats;

use std::error::Error;

use tracing::{info, error, warn, debug};

use crate::{rndr_reader::RndrReader, rndr_time::RndrTime};


#[derive(Debug, Clone, PartialEq, Default)]
struct RndrLog {
	date: String,
	time: String,
	status_code: String,
	msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();


    let (sx, rx) = channel();
    let watch_duration = Duration::from_secs(4);
	let mut watcher = watcher(sx, watch_duration).unwrap();

    let registry_data = RndrStats::get_registry_data().expect("could not get registry data");
    let rndrlog_watch_path = RndrReader::get_rndr_log_dir()?;
    info!("found watch path: {}", rndrlog_watch_path);

	watcher
		.watch(rndrlog_watch_path.to_string(), RecursiveMode::NonRecursive)
		.expect(
		"could not watch the file, something very wrong has happened.",
	);

    // NOTE THIS SHOULD NOT BE PRINTED, ONLY PASS DEBUG
	debug!("{:#?}", &registry_data);

	// NOTE this goes to the frontend through the app API,
	// but probably good to be logging it as well.
	info!(
		"minutes spent rendering: {:?}",
		RndrTime::total_time_in_minutes()
	);

    // listen for ctrl-c in the background.
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen to ctrl c");
        warn!("shutting down rndr watchtower");
        std::process::exit(0);

    });

        loop {
            match rx.recv() {
                /*
                * Send:
                * latest update RndrLog
                * total jobs completed
                */
                Ok(DebouncedEvent::Write(_)) => {
                    RndrReader::get_latest_update().expect("could not get latest update"); 	// NOTE this goes to the frontend of the app.

                    info!(
                        "minutes spent rendering: {:?}\n",
                        RndrTime::total_time_in_minutes()
                    );
                    println!("jobs completed: {}", registry_data.jobs_completed);
                }
                /*
                 * Send to front:
                 * RndrLog latest update
                 * total jobs completed.
                 */
                Ok(DebouncedEvent::Create(_)) => {
                    RndrReader::get_latest_update().unwrap();	// NOTE this goes to the frontend of the app.
                    info!(
                        "minutes spent rendering: {:?}\n",
                        RndrTime::total_time_in_minutes()
                    );
                }
                Ok(DebouncedEvent::NoticeWrite(path_buf)) => {
                    info!("received notice write: {:?}", path_buf);
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
