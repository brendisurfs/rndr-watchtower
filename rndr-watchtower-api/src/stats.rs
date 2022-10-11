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
	/// retrieves the RNDR data from the Windows Registry.
	pub fn get_registry_data() -> Result<RndrStats, Box<dyn std::error::Error>> {
		let rndr_registry = Hive::CurrentUser
			.open(r"Software\OTOY", Security::Read).expect("could not read rndr registry");

		// NOTE should expect these instead of an unwrap.
		let last_beta_timestamp = rndr_registry
			.value("LAST_BETA_LAUNCH_CHECK_TIMESTAMP")?;

		let should_launch_beta =
			rndr_registry.value("SHOULD_LAUNCH_BETA")?;

		let hw = rndr_registry.value("HW")?;
		let score = rndr_registry.value("SCORE")?;
		let node_id = rndr_registry.value("NODEID")?;
		let wallet_id = rndr_registry.value("WALLETID")?;
		let rndr_idle = rndr_registry.value("RNDR_IDLE")?;
		let starts_dual = rndr_registry.value("Starts_Dual")?;
		let runtime_dual = rndr_registry.value("Runtime_Dual")?;
		let runtime_rndr = rndr_registry.value("Runtime_RNDR")?;
		let restarts_rndr = rndr_registry.value("Restarts_RNDR")?;
		let jobs_completed = rndr_registry.value("JOBS_COMPLETED")?;

		let failure_metrics =
			rndr_registry.value("FAILURE_METRICS")?;

		let runtime_watchdog =
			rndr_registry.value("Runtime_Watchdog")?;

		let octane_gpu_settings =
			rndr_registry.value("OCTANE_GPU_SETTINGS")?;

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

		Ok(rndr_stats)
	}
}
