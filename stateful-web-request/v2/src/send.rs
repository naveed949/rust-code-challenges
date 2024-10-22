
#[derive(Debug, PartialEq)]
pub struct Send;

impl Send {
    pub fn send_response(&self) {
        println!("Response sent.");
    }
    pub fn connection_close(&self) {
        println!("Connection closed.");
    }
}
