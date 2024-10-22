
use crate::init::Initialize;
#[derive(Debug, PartialEq)]
pub enum State {
    Init,
    Routing,
    Dispatch,
    Send,
    Error,
}

impl State {
    pub fn new() -> State {
        State::Init
    }
    pub fn next(&self) -> State {
        match self {
            State::Init => State::Routing,
            State::Routing => State::Dispatch,
            State::Dispatch => State::Send,
            _ => State::Error,
        }
    }

    pub fn initialize(&self) -> State {
        match self {
            State::Init => {
                Initialize::initialize_services();
                Initialize::clients_connected();
                Initialize::configs_loaded();
                State::Routing
            },
            _ => {
                println!("Error: Cannot initialize from state {:?}", self);
                State::Error
            },
        }
    }
    
}
