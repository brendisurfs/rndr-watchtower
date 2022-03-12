use std::io::BufRead;

use crate::rndr_reader::RndrReader;

#[derive(Debug)]
pub struct RndrLog {
	pub date: String,
	pub time: String,
	pub status_code: String,
	pub msg: String,
}

#[derive(Debug)]
pub struct RndrTime;

impl RndrTime {
	pub fn get_total_rndr_time() -> std::io::Result<f64> {
		let mut rndr_times: Vec<f64> = vec![];

		let reader = RndrReader::new_log_reader();

		for line in reader.lines() {
			let split_line = line.as_ref().unwrap().splitn(4, " ");
			let msg = split_line.last().unwrap();
			let matched_lines = msg.matches("job completed successfully");
			for _ in matched_lines {
				let time_from_line = msg
					.splitn(8, " ")
					.last()
					.expect("could not go to last value in split")
					.split(" ")
					.skip(1)
					.next()
					.expect("error: could not go to next value in iteration")
					.parse::<f64>()
					.unwrap();
				rndr_times.push(time_from_line);
			}
		}
		let time_sum = rndr_times
			.into_iter()
			.reduce(|a, b| a + b)
			.expect("could not sum the vec of render times");
		Ok(time_sum)
	}

	///  time_in_minutes - calculates the time spent rendering in minutes.
	/// Returns f64
	pub fn time_in_minutes() -> f64 {
		let render_time = RndrTime::get_total_rndr_time();
		render_time.unwrap() / 60.0
	}

	pub fn check_new_event_update(line: String) {
		let mut split_line = line.splitn(4, " ");

		let log_date =
			split_line.next().expect("[ ERROR ] could not read date.");

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

		println!("{:#?}", new_line);
	}
}
