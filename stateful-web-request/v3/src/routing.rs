#[derive(Debug, PartialEq)]
pub struct Routing;

impl Routing {
    pub fn url_parse(&self) {
        println!("URL parsed.");
    }
    pub fn route_match(&self) {
        println!("Route matched.");
    }
    pub fn middleware_execute(&self) {
        println!("Middleware executed.");
    }
}
