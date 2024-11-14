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
    templates::cli_template::CliTemplate,
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
                if let Ok(project_type) = ProjectType::from_str(args.get("project_type").unwrap()) {
                    let writer= match project_type {
                        ProjectType::Cli => Box::new(Writer::new(CliTemplate)),
                        // Add other project types here
                        _ => {
                            error!("Invalid project type");
                            state = State::Error("Invalid project type".to_string());
                            continue;
                        }
                    };
                    if let Err(e) = writer.write_main_rs(&args.get("name").unwrap())
                        .and_then(|_| writer.write_mod_rs(&args.get("name").unwrap()))
                        .and_then(|_| writer.write_utils_rs(&args.get("name").unwrap()))
                        .and_then(|_| writer.write_error_rs(&args.get("name").unwrap())) {
                        error!("Failed to generate code templates: {}", e);
                        state = State::Error(e.to_string());
                    }
                } else {
                    error!("Invalid project type");
                    state = State::Error("Invalid project type".to_string());
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
