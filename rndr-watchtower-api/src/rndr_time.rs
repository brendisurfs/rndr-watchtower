use regex::Regex;
use std::io::BufRead;

use crate::rndr_reader::RndrReader;

#[derive(Debug)]
pub struct RndrLog {
	pub date: String,
	pub time: String,
	pub status_code: String,
	pub msg: String,
	pub render_time: f64,
}

#[derive(Debug)]
pub struct RndrTime;

impl RndrTime {
	pub fn get_total_rndr_time() -> std::io::Result<f64> {
		let mut rndr_times: Vec<f64> = vec![];

		let reader = RndrReader::new_log_reader().expect("could not get reader");

		for line in reader.lines() {
			let split_line = line.as_ref().unwrap().splitn(4, " ");
			let msg = split_line.last().unwrap();
			let matched_lines = msg.matches("job completed successfully");
			for _ in matched_lines {
				let time_from_line =
					RndrTime::get_render_time(&msg.to_string());
				rndr_times.push(time_from_line);
			}
		}
		let time_sum = rndr_times
			.into_iter()
			.reduce(|a, b| a + b)
			.expect("could not sum the vec of render times");

		let rounded_time = time_sum.round();
		Ok(rounded_time)
	}

	pub fn get_render_time(msg: &String) -> f64 {
		let local_msg = msg.clone();
		let time_from_line = local_msg
			.splitn(8, " ")
			.last()
			.unwrap_or(" ")
			.split(" ")
			.skip(1)
			.next()
			.unwrap_or("error: could not go to next value in iteration")
			.parse::<f64>()
			.unwrap_or_default();

		time_from_line
	}

	///  time_in_minutes - calculates the time spent rendering in minutes.
	/// Returns f64
	pub fn total_time_in_minutes() -> i64 {
		let render_time = RndrTime::get_total_rndr_time().expect("could not get total rendering time");
		let time_in_minutes = render_time / 60.0;
		time_in_minutes as i64
	}

	pub fn check_new_event_update(line: String) {
		let local_line = line.clone();
		let mut split_line = local_line.splitn(4, " ");

		let date_pattern = Regex::new(
			r"^[0-9]{4}-(((0[13578]|(10|12))-(0[1-9]|[1-2][0-9]|3[0-1]))|(02-(0[1-9]|[1-2][0-9]))|((0[469]|11)-(0[1-9]|[1-2][0-9]|30)))$",
		).unwrap();

		// should match date pattern
		let log_date_option = split_line.nth(0);
		let mut log_date_str = String::new();

		let is_logdate_valid =
			date_pattern.is_match(log_date_option.unwrap());

		if is_logdate_valid {
			log_date_str = log_date_option.unwrap().to_string();
		} else if !is_logdate_valid {
			log_date_str =
				"[ EERROR ] could not read log date.".to_string();
		}

		let log_time = split_line
			.next()
			.unwrap_or("[ ERROR ] could not read log time")
			.to_string();

		let log_status_code = split_line
			.next()
			.unwrap_or("[ ERROR ] could not read log status code")
			.split(":")
			.nth(0)
			.unwrap_or("no status code found")
			.to_string();

		let log_msg = split_line
			.next()
			.unwrap_or("[ ERROR ] could not read the log message")
			.to_string();

		let render_time = RndrTime::get_render_time(&log_msg).to_owned();

		let new_line = RndrLog {
			date: log_date_str,
			time: log_time,
			status_code: log_status_code,
			msg: log_msg,
			render_time,
		};

		println!("{:#?}", new_line);
	}
}
