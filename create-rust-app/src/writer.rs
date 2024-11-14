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
        match self.project_type {
            ProjectType::Cli => {
                // Write main.rs for CLI project
                match CliTemplate.write_main_rs(data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            ProjectType::Web => {
                // Write main.rs for Web project
                Ok(())
            }
            ProjectType::Desktop => {
                // Write main.rs for Desktop project
                Ok(())
            }
        }
    }

    fn write_mod_rs(&self, data: &str) -> Result<(), Error> {
        match self.project_type {
            ProjectType::Cli => {
                // Write mod.rs for CLI project
                match CliTemplate.write_mod_rs(data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            ProjectType::Web => {
                // Write mod.rs for Web project
                Ok(())
            }
            ProjectType::Desktop => {
                // Write mod.rs for Desktop project
                Ok(())
            }
        }
    }

    fn write_utils_rs(&self, data: &str) -> Result<(), Error> {
        match self.project_type {
            ProjectType::Cli => {
                // Write utils.rs for CLI project
                match CliTemplate.write_utils_rs(data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            ProjectType::Web => {
                // Write utils.rs for Web project
                Ok(())
            }
            ProjectType::Desktop => {
                // Write utils.rs for Desktop project
                Ok(())
            }
        }
    }

    fn write_error_rs(&self, data: &str) -> Result<(), Error> {
        match self.project_type {
            ProjectType::Cli => {
                // Write error.rs for CLI project
                match CliTemplate.write_error_rs(data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            ProjectType::Web => {
                // Write error.rs for Web project
                Ok(())
            }
            ProjectType::Desktop => {
                // Write error.rs for Desktop project
                Ok(())
            }
        }
    }
}