use crate::dispatch::Dispatch;
use crate::init::Initialize;
use crate::routing::Routing;
use crate::send::Send;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Init(Initialize),
    Routing(Routing),
    Dispatch(Dispatch),
    Send(Send),
    Error(String),
}

impl State {
    pub fn new(init: Initialize) -> State {
        State::Init(init)
    }
}
