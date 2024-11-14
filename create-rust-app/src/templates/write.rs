use std::io::Error;

pub trait Write {
    fn write_main_rs(&self, data: &str) -> Result<(), Error>;
    fn write_mod_rs(&self, data: &str) -> Result<(), Error>;
    fn write_utils_rs(&self, data: &str) -> Result<(), Error>;
    fn write_error_rs(&self, data: &str) -> Result<(), Error>;
}