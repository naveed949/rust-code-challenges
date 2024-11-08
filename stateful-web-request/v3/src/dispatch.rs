use crate::{request::Request, send::Send, state::State};

#[derive(Debug, PartialEq)]
pub struct Dispatch {
    request: Request,
}

impl Dispatch {
    pub fn from_routing(request: Request) -> Dispatch {
        Dispatch { request }
    }
    pub fn controller_execute(&self) {
        println!("Controller executed.");
    }
    pub fn prepare_response(&self) {
        println!("Preparing response.");
    }
    pub fn next(&self) -> State {
        State::Send(Send::from_dispatch(self.request.clone()))
    }
}
