extern crate directories;
use notify::{watcher, RecursiveMode, Watcher};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::mpsc::channel,
    time::Duration,
};

mod stats;

use stats::stats::get_rndr_stats;

use directories::BaseDirs;

#[derive(Debug, Clone, PartialEq, Default)]
struct RndrLog {
    date: String,
    time: String,
    status_code: String,
    msg: String,
}

impl RndrLog {
    fn new() -> Self {
        Default::default()
    }
}

const RNDR_LOG_EXTENSION: &str = "\\OtoyRndrNetwork\\rndr_log.txt";

fn main() {
    let (sender, receiver) = channel();

    // watches our rndr logs for any changes.
    let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();
    let watch_path = get_rndr_log_path().unwrap();
    watcher.watch(watch_path, RecursiveMode::Recursive).unwrap();

    read_rndr_log().unwrap();

    loop {
        match receiver.recv() {
            Ok(e) => {
                println!("[ NEW EVENT ] {:?}", &e);
                read_rndr_log().unwrap()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

pub type AppDataReturn = Result<String, String>;

// get_rndr_log_path - gets the path to the rndr logs from appdata on windows.
fn get_rndr_log_path() -> AppDataReturn {
    let base_dirs = BaseDirs::new().unwrap();
    let local_app_data_path = base_dirs.data_local_dir().to_str();

    match local_app_data_path {
        None => Err("error finding local data path".to_owned()),
        Some(datapath) => {
            let complete_appdata_path = format!("{}{}", datapath.to_owned(), RNDR_LOG_EXTENSION);
            Ok(complete_appdata_path)
        }
    }
}

// read_rndr_log - reads each line in the rndr log.
fn read_rndr_log() -> std::io::Result<()> {
    let log_file = get_rndr_log_path().expect("error: rndr log could not be found.");
    let file = File::open(log_file).expect("error: file not found!");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        check_new_event_update(line?)
    }
    Ok(())
}

fn check_new_event_update(line: String) {
    /*
    format: 2022-03-09 16:31:50 INFO: 1 usable gpus detected
    */

    let mut prev_line = RndrLog::new();

    let mut split_line = line.splitn(4, " ");
    let log_date = split_line.next().expect("[ ERROR ] could not read date.");
    let log_time = split_line
        .next()
        .expect("[ ERROR ] could not read the log time.");

    let log_status_code = split_line
        .next()
        .expect("[ ERROR ] could not read log status code");
    let log_msg = split_line
        .next()
        .expect("[ ERROR ] could not read the log message");

    let new_line = RndrLog {
        date: log_date.to_owned(),
        time: log_time.to_owned(),
        status_code: log_status_code.to_owned(),
        msg: log_msg.to_owned(),
    };

    // TODO - this section needs work on the equality, right now it isnt really helping.
    if prev_line == Default::default() {
        prev_line = new_line.clone();
        println!("{:?}", prev_line);
    } else if new_line != prev_line {
        println!("{:#?}", new_line);
    }
}
