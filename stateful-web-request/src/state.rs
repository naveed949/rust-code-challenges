#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Init,
    Routing,
    Dispatch,
    Send,
}

impl State {
    pub fn next(&self) -> State {
        match self {
            State::Init => State::Routing,
            State::Routing => State::Dispatch,
            State::Dispatch => State::Send,
            State::Send => State::Init,
        }
    }
}
