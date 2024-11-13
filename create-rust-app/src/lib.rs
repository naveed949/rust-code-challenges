pub mod cli;
pub mod state;
pub mod templates;

use std::fs;
use std::process::Command;
use log::info;
use std::error::Error;

pub fn create_project(name: &str, project_type: &str) -> Result<(), Box<dyn Error>> {
    info!("Creating project: {} of type: {}", name, project_type);
    Command::new("cargo")
        .arg("new")
        .arg(name)
        .arg(if project_type == "lib" { "--lib" } else { "--bin" })
        .status()?;
    Ok(())
}

pub fn create_directories(name: &str) -> Result<(), Box<dyn Error>> {
    let dirs = ["src", "tests", "examples", "benches"];
    for dir in &dirs {
        let path = format!("{}/{}", name, dir);
        info!("Creating directory: {}", path);
        fs::create_dir_all(&path)?;
    }
    Ok(())
}

pub fn create_config_files(name: &str, license: &str) -> Result<(), Box<dyn Error>> {
    info!("Creating configuration files for project: {}", name);
    fs::write(format!("{}/README.md", name), "# Project\n")?;
    fs::write(format!("{}/LICENSE", name), license)?;
    fs::write(format!("{}/.gitignore", name), "target/\n")?;
    Ok(())
}

pub fn add_dependencies(name: &str, dependencies: Option<&str>, dev_dependencies: Option<&str>) -> Result<(), Box<dyn Error>> {
    info!("Adding dependencies to project: {}", name);
    let mut cargo_toml = fs::read_to_string(format!("{}/Cargo.toml", name))?;
    if let Some(deps) = dependencies {
        cargo_toml.push_str(&format!("\n[dependencies]\n{}", deps.replace(",", "\n")));
    }
    if let Some(dev_deps) = dev_dependencies {
        cargo_toml.push_str(&format!("\n[dev-dependencies]\n{}", dev_deps.replace(",", "\n")));
    }
    fs::write(format!("{}/Cargo.toml", name), cargo_toml)?;
    Ok(())
}

