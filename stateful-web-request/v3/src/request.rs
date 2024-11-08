use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

#[derive(Debug, PartialEq)]
pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl Request {
    pub fn new() -> Self {
        Request {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, std::io::Error> {
        let mut reader = BufReader::new(stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let version = parts.next().unwrap_or("").to_string();

        let mut headers = Vec::new();
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            if line == "\r\n" {
                break;
            }
            let mut header_parts = line.splitn(2, ':');
            let key = header_parts.next().unwrap_or("").trim().to_string();
            let value = header_parts.next().unwrap_or("").trim().to_string();
            headers.push((key, value));
            line.clear();
        }

        let mut body = String::new();
        reader.read_to_string(&mut body)?;

        Ok(Request {
            method,
            path,
            version,
            headers,
            body: if body.is_empty() { None } else { Some(body) },
        })
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}
