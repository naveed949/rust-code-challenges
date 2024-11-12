use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;

use crate::dispatch::Dispatch;
use crate::init::Initialize;
use crate::routing::Routing;
use crate::send::Send;
use crate::state::State;

#[derive(Debug)]
pub struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
    stream: TcpStream,
    state: State,
}

impl Request {
    pub fn from_stream(stream: TcpStream) -> Self {
        let mut buf_reader = BufReader::new(&stream);
        let mut request_line = String::new();
        buf_reader.read_line(&mut request_line).unwrap();

        let mut headers = HashMap::new();
        let mut line = String::new();
        while buf_reader.read_line(&mut line).unwrap() > 0 {
            if line == "\r\n" {
                break;
            }
            let parts: Vec<&str> = line.split(": ").collect();
            headers.insert(parts[0].to_string(), parts[1].trim().to_string());
            line.clear();
        }

        let content_length = headers
            .get("Content-Length")
            .and_then(|cl| cl.parse::<usize>().ok())
            .unwrap_or(0);

        let mut body = vec![0; content_length];
        buf_reader.read_exact(&mut body).unwrap();
        let body = String::from_utf8(body).unwrap();

        let parts: Vec<&str> = request_line.split_whitespace().collect();
        Self {
            method: parts[0].to_string(),
            path: parts[1].to_string(),
            headers,
            body,
            stream,
            state: State::Init(Initialize::new()),
        }
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    pub fn send_response(&mut self, _response: &str) {
        // let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";
        self.stream.write_all(_response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
    pub fn state(&self) -> State {
        self.state.clone()
    }
    pub fn next_state(&mut self) {
        match self.state {
            State::Init(_) => {
                self.state = State::Routing(Routing);
            }
            State::Routing(_) => {
                self.state = State::Dispatch(Dispatch);
            }
            State::Dispatch(_) => {
                self.state = State::Send(Send);
            }
            State::Send(_) => {}
            State::Error(_) => {}
        }
    }

    pub fn path_params(&self) -> HashMap<String, String> {
        let parts: Vec<&str> = self.path.split('/').collect();
        let mut params = HashMap::new();
        let mut i = 0;
        for part in parts.clone() {
            if part.starts_with(':') {
                params.insert(part[1..].to_string(), parts[i + 1].to_string().clone());
            }
            i += 1;
        }
        params
    }

    pub fn query_params(&self) -> HashMap<String, String> {
        let parts: Vec<&str> = self.path.split('?').collect();
        let mut params = HashMap::new();
        if parts.len() > 1 {
            let query = parts[1];
            let pairs: Vec<&str> = query.split('&').collect();
            for pair in pairs {
                let kv: Vec<&str> = pair.split('=').collect();
                params.insert(kv[0].to_string(), kv[1].to_string());
            }
        }
        params
    }
}
