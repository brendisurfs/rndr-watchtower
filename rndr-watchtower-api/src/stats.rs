use registry::{Hive, Security};

pub struct RndrStats {
	wallet_id: String,
	node_id: String,
	pub rndr_idle: String,
	pub score: String,
	hw: String,
	octane_gpu_settings: String,
	runtime_watchdog: String,
	runtime_rndr: String,
	runtime_dual: String,
	restarts_rndr: String,
	starts_dual: String,
	pub jobs_completed: String,
}

#[derive(Debug)]
pub struct RegistryEntry {
	pub name: String,
	pub data: String,
}

impl RndrStats {
	pub fn new_data() {
		let rndr_registry = Hive::CurrentUser
			.open(r"Software\OTOY", Security::Read)
			.unwrap();

		let mut registry_vec: Vec<RegistryEntry> = vec![];

		rndr_registry.values().for_each(|v| {
			let registry_name = v.as_ref().unwrap().name().to_string().unwrap();
			let registry_data = v.as_ref().unwrap().data().to_string();

			let entry = RegistryEntry {
				name: registry_name,
				data: registry_data,
			};
			registry_vec.push(entry);
		});
		println!("{:#?}", registry_vec);
	}
}
