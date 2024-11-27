# Create Rust App

`create-rust-app` is a CLI tool designed to help you quickly set up a new Rust project with a predefined structure, dependencies, and configurations. It is inspired by `create-react-app` and aims to provide a similar experience for Rust developers.

## Features

- **Project Initialization**: Create a new Rust project with `cargo new`.
- **Directory Structure**: Set up common directories like `src`, `tests`, `examples`, and `benches`.
- **Configuration Files**: Generate common configuration files such as `.gitignore`, `README.md`, `LICENSE`, and `Cargo.toml`.
- **Dependencies**: Add common dependencies to `Cargo.toml` (e.g., `serde`, `tokio`, `reqwest`).
- **Code Templates**: Generate boilerplate code for main files, modules, and tests.
- **Customization**: Customize project metadata (e.g., project name, author, description) and choose a license type.
- **CI/CD Configuration**: Optionally include CI/CD configuration files (e.g., GitHub Actions, Travis CI).

## Installation

To install `create-rust-app`, you need to have Rust and Cargo installed on your system. You can install Rust and Cargo using [rustup](https://rustup.rs/).

```sh
cargo install create-rust-app
```

## Usage
To create a new Rust project, run the following command:

```sh
create-rust-app --name my_project --project-type cli --license MIT
```

### Command-Line Options

- `--name`: The name of the project.
- `--project-type`: The type of the project (cli or web).
- `--license`: The license type (MIT, Apache-2.0, etc.).
- `--include-ci`: Include CI/CD configuration (e.g., github for GitHub Actions).
- `--dependencies`: A comma-separated list of dependencies to add.
- `--dev-dependencies`: A comma-separated list of development dependencies to add.

### Example

```sh
create-rust-app --name example_project --project-type web --license Apache-2.0 --include-ci github --dependencies serde,log --dev-dependencies clippy,rustfmt
```

#### Project Structure

The generated project structure will look like this:

```
example_project/
├── Cargo.toml
├── LICENSE
├── README.md
├── .gitignore
├── src/
│   ├── main.rs
│   ├── mod.rs
│   ├── utils.rs
│   ├── error.rs
│   ├── server.rs (for web projects)
│   ├── router.rs (for web projects)
│   └── handlers.rs (for web projects)
├── tests/
├── examples/
└── benches/
```

#### Configuration Files
The following configuration files will be generated:
- `.gitignore`: Common Rust project ignores.
- `README.md`: A basic README file.
- `LICENSE`: The chosen license file.
- `Cargo.toml`: The Cargo configuration file with specified dependencies.

## Code Templates

The tool generates boilerplate code for the main files, modules, and tests. For example, a CLI project will have a `main.rs` file with basic CLI setup using clap, and a web project will have a `main.rs` file with basic web server setup using `actix-web`.

## Customization

You can customize the project metadata by specifying the project name, author, and description. You can also choose a license type and include CI/CD configuration files.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgements

This project is inspired by create-react-app and aims to provide a similar experience for Rust developers.

## Contact

For any questions or inquiries, please contact [me](https://github.com/naveed949) 🪄
