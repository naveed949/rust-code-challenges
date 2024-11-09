use crate::{request::Request, send::Send, state::State};

#[derive(Debug, PartialEq, Clone)]
pub struct Dispatch;

impl Dispatch {
    pub fn from_routing(request: Request) -> Dispatch {
        Dispatch
    }
    pub fn controller_execute(&self) {
        println!("Controller executed.");
    }
    pub fn prepare_response(&self) {
        println!("Preparing response.");
    }
    pub fn next(&self) -> &State {
        &State::Send(Send)
    }
}
