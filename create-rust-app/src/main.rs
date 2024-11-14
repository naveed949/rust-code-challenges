use create_rust_app::templates::write::Write;
use create_rust_app::writer::Writer;
use log::{error, info};
use std::error::Error;
use std::str::FromStr;
use structopt::StructOpt;

use create_rust_app::{add_dependencies, create_config_files, create_directories, create_project};
use create_rust_app::{
    cli::{Cli, ProjectType},
    state::State,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut state = State::Initialization;
    let args = Cli::from_args();

    loop {
        match state {
            State::Initialization => {
                if let Err(e) = create_project(&args.get("name").unwrap(), &String::from("bin")) {
                    error!("Failed to create project: {}", e);
                    state = State::Error(e.to_string());
                }
            }
            State::DirectoryStructure => {
                if let Err(e) = create_directories(&args.get("name").unwrap()) {
                    error!("Failed to create directories: {}", e);
                    state = State::Error(e.to_string());
                }
            }
            State::ConfigurationFiles => {
                if let Err(e) = create_config_files(
                    &args.get("name").unwrap(),
                    args.get("license").unwrap_or(&String::from("MIT")),
                ) {
                    error!("Failed to create configuration files: {}", e);
                    state = State::Error(e.to_string());
                }
            }
            State::Dependencies => {
                if let Err(e) = add_dependencies(
                    &args.get("name").unwrap(),
                    args.get("dependencies").as_deref().map(String::as_str),
                    args.get("dev_dependencies").as_deref().map(String::as_str),
                ) {
                    error!("Failed to add dependencies: {}", e);
                    state = State::Error(e.to_string());
                }
            }
            State::CodeTemplates => {
                // Add code template generation logic here
                ProjectType::from_str(args.get("project_type").unwrap())
                    .map(|project_type| -> Result<(), Box<dyn Error>> {
                        match project_type {
                            ProjectType::Cli => {
                                // Add CLI code template generation logic here
                                let writer = Writer::new(ProjectType::Cli);
                                writer.write_main_rs(&args.get("name").unwrap())?; // TODO: need to correct root path
                                writer.write_mod_rs(&args.get("name").unwrap())?;
                                writer.write_utils_rs(&args.get("name").unwrap())?;
                                writer.write_error_rs(&args.get("name").unwrap())?;
                            }
                            ProjectType::Web => {
                                // Add web code template generation logic here
                                let writer = Writer::new(ProjectType::Web);
                                writer.write_main_rs("")?;
                                writer.write_mod_rs("")?;
                                writer.write_utils_rs("")?;
                                writer.write_error_rs("")?;
                            }
                            ProjectType::Desktop => {
                                // Add desktop code template generation logic here
                                let writer = Writer::new(ProjectType::Desktop);
                                writer.write_main_rs("")?;
                                writer.write_mod_rs("")?;
                                writer.write_utils_rs("")?;
                                writer.write_error_rs("")?;
                            }
                        }
                        Ok(())
                    })
                    .unwrap_or_else(|e| {
                        error!("Failed to generate code templates: {}", e);
                        state = State::Error(e.to_string());
                        Ok(())
                    })?;
            }
            State::Customization => {
                // Add customization logic here
                state = State::Finalization;
            }
            State::Finalization => {
                info!("Project setup complete.");
                state = State::Done;
            }
            State::Done => {
                break;
            }
            State::Error(ref msg) => {
                error!("Error encountered: {}", msg);
                return Err(msg.clone().into());
            }
        }
        state = state.next();
    }
    Ok(())
}
