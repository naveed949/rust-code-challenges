
        use crate::error::CustomError;
        use clap::{App, Arg};

        pub fn greet(name: &str) -> Result<(), CustomError> {
            println!("Hello, {}!", name);
            Ok(())
        }
        