mod dispatch;
mod init;
mod routing;
mod send;
mod state;

mod request;

use dispatch::Dispatch;
use init::Initialize;
use request::Request;
use routing::Routing;
use send::Send;
use state::State;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use threadpool::ThreadPool;

// TODO: Implement it in Send state
fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on port 7878");

    let web_server_pool = ThreadPool::new(4);
    let state_machine_pool = ThreadPool::new(5); // 1 + 4. 1 for the parent thread and 4 for the worker threads

    let (tx, rx) = mpsc::channel();

    web_server_pool.execute(move || {
        // parent thread <1> of the state machine
        println!("State machine running...");
        loop {
            // infinite loop to keep the state machine running
            let mut req: Request = rx.recv().unwrap_or_else(|e| {
                State::Error(format!("Error: {}", e));
                panic!("Error: {}", e);
            });
            state_machine_pool.execute(move || {
                // worker thread <2> of the state machine per message received
                // println!("State: {:?}", req.state());
                loop {
                    let state = req.state();
                    match state {
                        State::Init(init) => {
                            init.initialize_services();
                            init.configs_loaded();
                            init.clients_connected();
                            req.next_state();
                        }
                        State::Routing(routing) => {
                            routing.url_parse();
                            routing.route_match();
                            routing.middleware_execute();
                            req.next_state();
                        }
                        State::Dispatch(dispatch) => {
                            dispatch.controller_execute();
                            dispatch.prepare_response();
                            req.next_state();
                        }
                        State::Send(send) => {
                            let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
                            send.send_response(&mut req, response);
                            send.connection_close();
                            // send.next();
                            break;
                        }
                        State::Error(e) => {
                            println!("Error: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    });

    // main thread <0> of the web server to accept incoming connections
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let tx = tx.clone();
        web_server_pool.execute(move || {
            let req = Request::from_stream(stream);

            tx.send(req).unwrap_or_else(|e| {
                panic!("Error: {}", e);
            });
        });
    }
}
