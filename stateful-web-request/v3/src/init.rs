use crate::{request::Request, routing::Routing, state::State};

// Initialises the services, loads the configs and notifies when clients are connected.
#[derive(Debug, PartialEq)]
pub struct Initialize {
    request: Request,
}

impl Initialize {
    pub fn new(req: Request) -> Initialize {
        Initialize { request: req }
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

    pub fn next(&self) -> State {
        State::Routing(Routing::from_init(self.request.clone()))
    }
}
