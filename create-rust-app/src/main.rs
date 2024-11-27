use create_rust_app::templates::write::{WebWrite, Write};
use log::{error, info};
use std::error::Error;
use std::str::FromStr;
use structopt::StructOpt;

use create_rust_app::{add_dependencies, create_config_files, create_directories, create_project};
use create_rust_app::{
    cli::{Cli, ProjectType},
    state::State,
    templates::cli_template::CliTemplate,
    templates::web_template::WebTemplate,
};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let mut state = State::Initialization;
    let args = Cli::from_args();

    loop {
        match state {
            State::Initialization => {
                if let Err(e) = create_project(&args.get("name").unwrap(), "bin") {
                    error!("Failed to create project: {}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::DirectoryStructure;
                }
            }
            State::DirectoryStructure => {
                if let Err(e) = create_directories(&args.get("name").unwrap()) {
                    error!("Failed to create directories: {}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::ConfigurationFiles;
                }
            }
            State::ConfigurationFiles => {
                if let Err(e) = create_config_files(
                    &args.get("name").unwrap(),
                    args.get("license").unwrap_or(&String::from("MIT")),
                ) {
                    error!("Failed to create configuration files: {}", e);
                    state = State::Error(e.to_string());
                } else {
                    state = State::Dependencies;
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
                } else {
                    state = State::CodeTemplates;
                }
            }
            State::CodeTemplates => {
                let project_type = ProjectType::from_str(args.get("project_type").unwrap());
                match project_type {
                    Ok(project) => {
                        match project {
                            ProjectType::Cli => {
                                let template = CliTemplate;
                                template.write_main_rs(&args.get("name").unwrap())?;
                                template.write_mod_rs(&args.get("name").unwrap())?;
                                template.write_utils_rs(&args.get("name").unwrap())?;
                                template.write_error_rs(&args.get("name").unwrap())?;
                            }
                            ProjectType::Web => {
                                let template = WebTemplate;
                                template.write_main_rs(&args.get("name").unwrap())?;
                                template.write_mod_rs(&args.get("name").unwrap())?;
                                template.write_utils_rs(&args.get("name").unwrap())?;
                                template.write_error_rs(&args.get("name").unwrap())?;
                                template.write_server_rs(&args.get("name").unwrap())?;
                                template.write_router_rs(&args.get("name").unwrap())?;
                                template.write_handlers_rs(&args.get("name").unwrap())?;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse project type: {}", e);
                        state = State::Error(e.to_string());
                    }
                    
                }
            }
            State::Customization => {
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
