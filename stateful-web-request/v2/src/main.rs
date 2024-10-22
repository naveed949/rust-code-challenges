mod init;
mod routing;
mod dispatch;
mod send;
mod state;

use state::State;
fn main() {
    let mut state = State::new();
    loop {
        state = state.initialize();
        state = state.next();
        println!("Next state: {:?}", state);
        if state == State::Error {
            break;
        }
    }
}
