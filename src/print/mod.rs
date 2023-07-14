
pub fn info<T>(a: &T) where T: std::fmt::Debug{
    println!("{:?}",a);
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);

    // Advanced 1
    // use std::ffi::CString;
    
    // let c = CString::new("?").unwrap();
    // info(&input);

    // Advanced 2
    // use std::path::Path;
    // let d = Path::new("/tmp/linkedin-learning");
    // info(d);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

#[test]
fn chars() {
    let input = 'r';
    info(&input);
}

#[test]
fn cstring() {
    use std::ffi::{CString};
    let input = CString::new("Rust").unwrap();
    info(&input);
}

#[test]
fn path() {
    use std::path::PathBuf;
    let input = "/tmp/rust";
    let input_str = std::str::from_utf8(input.as_bytes()).unwrap();
    let path = PathBuf::from(input_str);
    info(&path);
}