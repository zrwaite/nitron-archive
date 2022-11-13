pub struct NitroGalaxy {
	tron: TronSystem,
}
impl NitroGalaxy {
	pub fn new() -> Self {
		Self {
			tron: TronSystem::new(),
		}
	}
}

pub struct TronSystem {
	pub nitron: PlanetNitron,
}
impl TronSystem {
	pub fn new() -> Self {
		Self {
			nitron: PlanetNitron::new(),
		}
	}
}

pub struct PlanetNitron {
	pub nitron_city: String,
}
impl PlanetNitron {
	pub fn new() -> Self {
		Self {
			nitron_city: "nitron_city".to_string(),
		}
	}
}

