mod dispatch;
mod init;
mod routing;
mod send;
mod state;

use dispatch::Dispatch;
use init::Initialize;
use routing::Routing;
use send::Send;
use state::State;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server running on port 7878");

    let web_server_pool = ThreadPool::new(4);
    let state_machine_pool = ThreadPool::new(4);

    let (tx, rx) = mpsc::channel();

    web_server_pool.execute(move || {
        // parent thread <1> of the state machine
        println!("State machine running...");
        loop {
            // infinite loop to keep the state machine running
            let state = rx.recv().unwrap_or_else(|e| {
                State::Error(format!("Error: {}", e));
                panic!("Error: {}", e);
            });
            state_machine_pool.execute(move || {
                // worker thread <2> of the state machine per message received
                println!("State: {:?}", state);
                let mut state = state;
                loop {
                    match state {
                        State::Init(init) => {
                            init.initialize_services();
                            init.configs_loaded();
                            init.clients_connected();
                            state = State::Routing(Routing);
                        }
                        State::Routing(routing) => {
                            routing.url_parse();
                            routing.route_match();
                            routing.middleware_execute();
                            state = State::Dispatch(Dispatch);
                        }
                        State::Dispatch(dispatch) => {
                            dispatch.controller_execute();
                            dispatch.prepare_response();
                            state = State::Send(Send);
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
            });
        }
    });

    // main thread <0> of the web server to accept incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let tx = tx.clone();
        web_server_pool.execute(move || {
            handle_connection(stream);
            println!("Connection established.");
            tx.send(State::Init(Initialize::new())).unwrap_or_else(|e| {
                State::Error(format!("Error: {}", e));
                panic!("Error: {}", e);
            });
        });
    }
}
