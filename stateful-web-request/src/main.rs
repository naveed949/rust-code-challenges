mod dispatch;
mod init;
mod routing;
mod send;
mod state;

use dispatch::Dispatch;
use routing::Routing;
use send::Send;
use state::State;

fn main() {
    let mut state = State::Init;
    loop {
        match state {
            State::Init => {
                init::Initial::initialise_services();
                init::Initial::clients_connected();
                init::Initial::configs_loaded();
            }
            State::Routing => {
                Routing::url_parse();
                Routing::route_match();
                Routing::middleware_execute();
            }
            State::Dispatch => {
                Dispatch::controller_execute();
                Dispatch::prepare_response();
            }
            State::Send => {
                Send::send_response();
                Send::connection_close();
                break;
            }
        }
        state = state.next();
        println!("> Next state: {:?}", state);
    }
}
