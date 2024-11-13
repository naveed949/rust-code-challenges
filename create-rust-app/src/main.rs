use structopt::StructOpt;
use log::{info, error};
use std::error::Error;

use create_rust_app::{create_project, create_directories, create_config_files, add_dependencies};
use create_rust_app::{cli::Cli, state::State};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut state = State::Initialization;
    let args = Cli::from_args();
    
    loop {
        match state {
            State::Initialization => {
                if let Err(e) = create_project(
                    &args.get("name").unwrap(),
                    args.get("project_type").unwrap_or(&String::from("bin")),
                ) {
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
                state = State::Customization;
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
