use std::io::Error;

use crate::cli::ProjectType;
use crate::templates::write::Write;
use crate::templates::cli_template::CliTemplate;

pub struct Writer<T> {
    project_type: T,
}

impl<T> Writer<T> {
    pub fn new(project_type: T) -> Self {
        Writer { project_type }
    }
}

impl Write for Writer<ProjectType> {
    fn write_main_rs(&self, data: &str) -> Result<(), Error> {
        if let ProjectType::Cli = self.project_type {
            CliTemplate.write_main_rs(data)
        } else {
            Ok(())
        }
    }

    fn write_mod_rs(&self, data: &str) -> Result<(), Error> {
        if let ProjectType::Cli = self.project_type {
            CliTemplate.write_mod_rs(data)
        } else {
            Ok(())
        }
    }

    fn write_utils_rs(&self, data: &str) -> Result<(), Error> {
        if let ProjectType::Cli = self.project_type {
            CliTemplate.write_utils_rs(data)
        } else {
            Ok(())
        }
    }

    fn write_error_rs(&self, data: &str) -> Result<(), Error> {
        if let ProjectType::Cli = self.project_type {
            CliTemplate.write_error_rs(data)
        } else {
            Ok(())
        }
    }
}