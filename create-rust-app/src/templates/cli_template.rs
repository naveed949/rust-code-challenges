use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::io::{Error, Result};

pub struct CliTemplate;

impl CliTemplate {
    pub fn write_main_rs(&self, name: &str) -> Result<()> {
        let code = r#"
    mod utils;
    mod error;

    fn main() -> Result<(), CustomError> {
        let matches = App::new("Hello World CLI")
            .version("1.0")
            .author("Your Name <your.email@example.com>")
            .about("Says hello")
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .help("Sets the name to greet")
                    .takes_value(true),
            )
            .arg(Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true))
            .get_matches();

        let name = matches.value_of("name").unwrap_or("world");
        utils::greet(name)?;
        Ok(())
    }"#;
        let dir = format!("{}/src/main.rs", name);
    self.generate_rust_code(&dir, code)
    }

    pub fn write_mod_rs(&self, name: &str) -> std::io::Result<()> {
        let code = r#"
        pub mod utils;
        pub mod error;
        "#;
        let dir = format!("{}/src/mod.rs", name);
        self.generate_rust_code(&dir, code)
    }

    pub fn write_utils_rs(&self, name: &str) -> std::io::Result<()> {
        let code = r#"
        use crate::error::CustomError;
        use clap::{App, Arg};

        pub fn greet(name: &str) -> Result<(), CustomError> {
            println!("Hello, {}!", name);
            Ok(())
        }
        "#;
        let dir = format!("{}/src/utils.rs", name);
        self.generate_rust_code(&dir, code)
    }

    pub fn write_error_rs(&self, name: &str) -> std::io::Result<()> {
        let code = r#"
        use thiserror::Error;

        #[derive(Error, Debug)]
        pub enum CustomError {
            #[error("An error occurred.")]
            AnError,
        }
        "#;
        let dir = format!("{}/src/error.rs", name);
        self.generate_rust_code(&dir, code)
    }

    fn generate_rust_code(&self, filename: &str, code: &str) -> std::io::Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        file.write_all(code.as_bytes())?;
        Ok(())
    }
}
