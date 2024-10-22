#[derive(Debug)]
pub enum State {
    Init,
    Routing,
    Dispatch,
    Send,
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
            State::Send => State::Init,
        }
    }
    
}
