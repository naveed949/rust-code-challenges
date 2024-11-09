use crate::{request::Request, state::State};

#[derive(Debug, PartialEq, Clone)]
pub struct Send;

impl Send {
    pub fn from_dispatch(_request: Request) -> Send {
        Send
    }
    pub fn send_response(&self, _request: &mut Request, _response: &str) {
        // let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
        _request.send_response(_response);
        println!("Response sent.");
    }
    pub fn connection_close(&self) {
        println!("Connection closed.");
    }
    pub fn next(&self) -> State {
        State::Error("No state".to_string())
    }
}
