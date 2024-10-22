pub struct Send;

impl Send {
    pub fn send_response() {
        println!("Response sent.");
    }
    pub fn connection_close() {
        println!("Connection closed.");
    }
}
