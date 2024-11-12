# create-rusty-app

## Project Initialization
- Create new Rust project
  - `cargo new`
  - Options:
    - Binary (`--bin`)
    - Library (`--lib`)

## Directory Structure
- Common directories
  - `src`
  - `tests`
  - `examples`
  - `benches`
- Additional directories (optional)
  - `docs`
  - `scripts`

## Configuration Files
- Generate common configuration files
  - `.gitignore`
  - `README.md`
  - `LICENSE`
  - `Cargo.toml`
- CI/CD configuration files (optional)
  - GitHub Actions
  - Travis CI

## Dependencies
- Add common dependencies to `Cargo.toml`
  - `serde`
  - `tokio`
  - `reqwest`
- Development dependencies (optional)
  - `clippy`
  - `rustfmt`

## Code Templates
- Generate boilerplate code
  - Main files
  - Modules
  - Tests
- Example code for common patterns (optional)
  - Error handling
  - Logging

## Customization
- Project metadata
  - Project name
  - Author
  - Description
- License type
  - MIT
  - Apache 2.0
  - Other licenses

## CLI Options
- Command-line arguments and options
  - `--name` (project name)
  - `--project-type` (binary or library)
  - `--license` (license type)
  - `--include-ci` (include CI/CD configuration)
  - `--dependencies` (list of dependencies)
  - `--dev-dependencies` (list of development dependencies)

## Implementation Plan
- Set up the CLI tool
  - Use `structopt` or `clap` crate to handle command-line arguments and options
  - Create a new Rust project for the CLI tool
- Project initialization
  - Implement a function to run `cargo new` with the specified options
  - Handle both binary and library project types
- Directory structure
  - Implement functions to create common directories
  - Use `std::fs` to create directories and files
- Configuration files
  - Implement functions to generate common configuration files
  - Use templates for files like `.gitignore`, `README.md`, and `LICENSE`
- Dependencies
  - Implement functions to modify `Cargo.toml` and add dependencies
  - Use `toml` crate to parse and modify `Cargo.toml`
- Code templates
  - Implement functions to generate boilerplate code for main files, modules, and tests
  - Use templates for common code patterns
- Customization
  - Implement options to customize project metadata
  - Allow users to choose a license type and generate the corresponding file

## Example Usage
- Initialize a new project
  - `create-rusty-app --name my_project --project-type bin --license MIT`
- Include CI/CD configuration
  - `create-rusty-app --name my_project --include-ci github-actions`
- Add dependencies
  - `create-rusty-app --name my_project --dependencies serde,tokio --dev-dependencies clippy,rustfmt`

## Detailed Implementation Plan

### 1. Set Up the CLI Tool
- **Create a new Rust project**:
  - `cargo new create-rusty-app --bin`
- **Add dependencies**:
  - Add `structopt` or `clap` crate to `Cargo.toml` for handling CLI arguments.

### 2. Project Initialization
- **Function to run `cargo new`**:
  - Implement a function that executes `cargo new` with the specified project name and type (binary or library).
  - Use `std::process::Command` to run the `cargo new` command.

### 3. Directory Structure
- **Create common directories**:
  - Implement functions to create directories like `src`, `tests`, `examples`, and `benches`.
  - Use `std::fs::create_dir_all` to create directories.

### 4. Configuration Files
- **Generate common configuration files**:
  - Implement functions to create `.gitignore`, `README.md`, `LICENSE`, and `Cargo.toml`.
  - Use templates for these files and fill in project-specific details (e.g., project name, author).

### 5. CI/CD Configuration Files
- **Optional CI/CD configuration**:
  - Implement functions to create CI/CD configuration files for GitHub Actions, Travis CI, etc.
  - Use templates for these files and customize them based on user input.

### 6. Dependencies
- **Modify `Cargo.toml`**:
  - Implement functions to add dependencies to `Cargo.toml`.
  - Use the `toml` crate to parse and modify `Cargo.toml`.

### 7. Code Templates
- **Generate boilerplate code**:
  - Implement functions to create main files, modules, and tests with boilerplate code.
  - Use templates for common code patterns (e.g., error handling, logging).

### 8. Customization
- **Project metadata**:
  - Implement options to customize project metadata (e.g., project name, author, description).
  - Allow users to choose a license type and generate the corresponding file.

### 9. CLI Options
- **Handle command-line arguments**:
  - Use `structopt` or `clap` to define and parse command-line arguments and options.
  - Implement options for project name, type, license, dependencies, etc.

### 10. Example Usage
- **Initialize a new project**:
  - `create-rusty-app --name my_project --project-type bin --license MIT`
- **Include CI/CD configuration**:
  - `create-rusty-app --name my_project --include-ci github-actions`
- **Add dependencies**:
  - `create-rusty-app --name my_project --dependencies serde,tokio --dev-dependencies clippy,rustfmt`