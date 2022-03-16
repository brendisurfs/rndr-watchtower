use req::StatusCode;
use reqwest as req;

use crate::stats::RndrStats;

pub async fn send_node_credentials(reg_data: &RndrStats) {
	let response = req::get("127.0.0.1:8080").await.unwrap();
	match response.status() {
		StatusCode::OK => {
			println!("status ok!")
		}
		StatusCode::UNAUTHORIZED => {
			println!("unauthorized: please make sure your node data is valid.")
		}
		_ => (),
	}
}
