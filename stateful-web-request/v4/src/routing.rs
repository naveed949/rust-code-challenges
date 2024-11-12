use crate::{dispatch::Dispatch, request::Request, state::State};

#[derive(Debug, PartialEq, Clone)]
pub struct Routing;

impl Routing {
    pub fn from_init(request: Request) -> Routing {
        Routing
    }
    pub fn url_parse(&self) {
        println!("URL parsed.");
    }
    pub fn route_match(&self) {
        println!("Route matched.");
    }
    pub fn middleware_execute(&self) {
        println!("Middleware executed.");
    }
    pub fn next(&self) -> &State {
        &State::Dispatch(Dispatch)
    }
}
