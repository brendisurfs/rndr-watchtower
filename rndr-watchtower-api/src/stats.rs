use registry::{Hive, Security};

#[derive(Debug)]
pub struct RndrStats {
	pub hw: String,
	pub score: String,
	pub node_id: String,
	pub wallet_id: String,
	pub rndr_idle: String,
	pub starts_dual: String,
	pub runtime_rndr: String,
	pub runtime_dual: String,
	pub restarts_rndr: String,
	pub jobs_completed: String,
	pub failure_metrics: String,
	pub runtime_watchdog: String,
	pub should_launch_beta: String,
	pub octane_gpu_settings: String,
	pub last_beta_launch_check_timestamp: String,
}

impl RndrStats {
	/// new_data - retrieves the RNDR data from the Windows Registry.
	pub fn get_registry_data() -> RndrStats {
		let rndr_registry = Hive::CurrentUser
			.open(r"Software\OTOY", Security::Read)
			.unwrap();

		// NOTE should expect these instead of an unwrap.
		let last_beta_timestamp = rndr_registry
			.value("LAST_BETA_LAUNCH_CHECK_TIMESTAMP")
			.unwrap();

		let should_launch_beta =
			rndr_registry.value("SHOULD_LAUNCH_BETA").unwrap();

		let hw = rndr_registry.value("HW").unwrap();
		let score = rndr_registry.value("SCORE").unwrap();
		let node_id = rndr_registry.value("NODEID").unwrap();
		let wallet_id = rndr_registry.value("WALLETID").unwrap();
		let rndr_idle = rndr_registry.value("RNDR_IDLE").unwrap();
		let starts_dual = rndr_registry.value("Starts_Dual").unwrap();
		let runtime_dual = rndr_registry.value("Runtime_Dual").unwrap();
		let runtime_rndr = rndr_registry.value("Runtime_RNDR").unwrap();
		let restarts_rndr = rndr_registry.value("Restarts_RNDR").unwrap();
		let jobs_completed = rndr_registry.value("JOBS_COMPLETED").unwrap();
		let failure_metrics =
			rndr_registry.value("FAILURE_METRICS").unwrap();
		let runtime_watchdog =
			rndr_registry.value("Runtime_Watchdog").unwrap();

		let octane_gpu_settings =
			rndr_registry.value("OCTANE_GPU_SETTINGS").unwrap();

		let rndr_stats = RndrStats {
			hw: hw.to_string(),
			score: score.to_string(),
			node_id: node_id.to_string(),
			rndr_idle: rndr_idle.to_string(),
			wallet_id: wallet_id.to_string(),
			starts_dual: starts_dual.to_string(),
			runtime_dual: runtime_dual.to_string(),
			runtime_rndr: runtime_rndr.to_string(),
			restarts_rndr: restarts_rndr.to_string(),
			jobs_completed: jobs_completed.to_string(),
			failure_metrics: failure_metrics.to_string(),
			runtime_watchdog: runtime_watchdog.to_string(),
			should_launch_beta: should_launch_beta.to_string(),
			octane_gpu_settings: octane_gpu_settings.to_string(),
			last_beta_launch_check_timestamp: last_beta_timestamp
				.to_string(),
		};

		rndr_stats
	}
}
