use std::io::Error;

use crate::cli::ProjectType;
use crate::templates::write::Write;

pub struct Writer<T: Write> {
    project_type: T,
}

impl<T: Write> Writer<T> {
    pub fn new(project_type: T) -> Self {
        Writer { project_type }
    }

    pub fn write_main_rs(&self, data: &str) -> Result<(), Error> {
        self.project_type.write_main_rs(data)
    }

    pub fn write_mod_rs(&self, data: &str) -> Result<(), Error> {
        self.project_type.write_mod_rs(data)
    }

    pub fn write_utils_rs(&self, data: &str) -> Result<(), Error> {
        self.project_type.write_utils_rs(data)
    }

    pub fn write_error_rs(&self, data: &str) -> Result<(), Error> {
        self.project_type.write_error_rs(data)
    }
}