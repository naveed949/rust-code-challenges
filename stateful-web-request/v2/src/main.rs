mod init;
mod routing;
mod dispatch;
mod send;
mod state;

use init::Initialize;
use state::State;
fn main() {
    let init = Initialize::new();
    let mut state = State::new(init);
    loop {
        match state {
            State::Init(init) => {
                init.initialize_services();
                init.configs_loaded();
                init.clients_connected();
                state = State::Routing(routing::Routing);
            }
            State::Routing(routing) => {
                routing.url_parse();
                routing.route_match();
                routing.middleware_execute();
                state = State::Dispatch(dispatch::Dispatch);
            }
            State::Dispatch(dispatch) => {
                dispatch.controller_execute();
                dispatch.prepare_response();
                state = State::Send(send::Send);
            }
            State::Send(send) => {
                send.send_response();
                send.connection_close();
                break;
            }
            State::Error(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}
