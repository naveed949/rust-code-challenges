use std::fs::File;
use std::io::Write as IoWrite;
use std::path::Path;
use std::io::Result;

use crate::templates::write::{Write, WebWrite};

pub struct WebTemplate;

impl Write for WebTemplate {
    fn write_main_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        mod server;
        mod router;
        mod handlers;

        fn main() -> std::io::Result<()> {
            server::run()
        }
        "#;
        self.generate_rust_code(&format!("{}/src/main.rs", name), code)
    }

    fn write_mod_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        pub mod server;
        pub mod router;
        pub mod handlers;
        "#;
        self.generate_rust_code(&format!("{}/src/mod.rs", name), code)
    }

    fn write_utils_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        // No utility functions for this template
        "#;
        self.generate_rust_code(&format!("{}/src/utils.rs", name), code)
    }

    fn write_error_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        // No custom errors for this template
        "#;
        self.generate_rust_code(&format!("{}/src/error.rs", name), code)
    }

}

impl WebWrite for WebTemplate {
    fn write_server_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        use actix_web::{App, HttpServer};

        use crate::router::configure;

        pub async fn run() -> std::io::Result<()> {
            HttpServer::new(|| {
                App::new()
                    .configure(configure)
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        }
        "#;
        self.generate_rust_code(&format!("{}/src/server.rs", name), code)
    }

    fn write_router_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        use actix_web::web;

        use crate::handlers::greet;

        pub fn configure(cfg: &mut web::ServiceConfig) {
            cfg.route("/", web::get().to(greet));
        }
        "#;
        self.generate_rust_code(&format!("{}/src/router.rs", name), code)
    }

    fn write_handlers_rs(&self, name: &str) -> Result<()> {
        let code = r#"
        use actix_web::{Responder};

        pub async fn greet() -> impl Responder {
            format!("Hello, world!")
        }
        "#;
        self.generate_rust_code(&format!("{}/src/handlers.rs", name), code)
    }
}

impl WebTemplate {
    fn generate_rust_code(&self, filename: &str, code: &str) -> Result<()> {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        file.write_all(code.as_bytes())
    }
}