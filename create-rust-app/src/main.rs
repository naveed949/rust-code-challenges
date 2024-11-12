use structopt::StructOpt;
use log::info;
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
                create_project(
                    &args.get("name").unwrap(),
                    args.get("project_type").unwrap_or(&String::from("bin")),
                )?;
            }
            State::DirectoryStructure => {
                
                create_directories(&args.get("name").unwrap())?;
            }
            State::ConfigurationFiles => {
                create_config_files(
                    &args.get("name").unwrap(),
                    args.get("license").unwrap_or(&String::from("MIT")),
                )?;
            }
            State::Dependencies => {
                add_dependencies(
                    &args.get("name").unwrap(),
                    args.get("dependencies").as_deref().map(String::as_str),
                    args.get("dev_dependencies").as_deref().map(String::as_str),
                )?;
            }
            State::CodeTemplates => {
                state = State::Customization;
            }
            State::Customization => {
                state = State::Finalization;
            }
            State::Finalization => {
                state = State::Done;
            }
            State::Done => {
                break;
            }
            State::Error(ref msg) => {
                return Err(msg.clone().into());
            }
        }
        state = state.next(); 
    }
    Ok(())
}
