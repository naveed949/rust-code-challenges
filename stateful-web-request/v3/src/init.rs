// Initialises the services, loads the configs and notifies when clients are connected.
#[derive(Debug, PartialEq)]
pub struct Initialize;

impl Initialize {
    pub fn new() -> Initialize {
        Initialize
    }
    pub fn initialize_services(&self) {
        println!("Initialising services...");
    }

    pub fn clients_connected(&self) {
        println!("Clients connected.");
    }

    pub fn configs_loaded(&self) {
        println!("Configs loaded.");
    }
}
