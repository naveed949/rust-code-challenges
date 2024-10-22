
#[derive(Debug, PartialEq)]
pub struct Dispatch;

impl Dispatch {
    pub fn controller_execute(&self) {
        println!("Controller executed.");
    }
    pub fn prepare_response(&self) {
        println!("Preparing response.");
    }
}
