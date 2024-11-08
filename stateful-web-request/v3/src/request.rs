use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug, PartialEq, Clone)]
pub struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Request {
    pub fn from_stream(stream: &TcpStream) -> Self {
        let mut buf_reader = BufReader::new(stream);
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
}
