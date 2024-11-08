use crate::{request::Request, state::State};

#[derive(Debug, PartialEq)]
pub struct Send {
    request: Request,
}

impl Send {
    pub fn from_dispatch(_request: Request) -> Send {
        Send { request: _request }
    }
    pub fn send_response(&self) {
        println!("Response sent.");
    }
    pub fn connection_close(&self) {
        println!("Connection closed.");
    }
    pub fn next(&self) -> State {
        State::Error("No state".to_string())
    }
}
