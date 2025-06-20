use crate::{
    templates::{env, readme},
    utils::{install_dependency, render_template},
};
use rstack_macros::{
    generate_actix_main, generate_actix_routes, generate_axum_main, generate_axum_routes,
};
use serde::Serialize;
use std::{
    fs::{self, create_dir_all},
    io,
    path::Path,
    process::Command,
};

#[derive(Debug, Serialize)]
pub struct Config {
    dir: String,
    name: String,
    server: String,
    db: String,
    orm: String,
}

impl Config {
    pub fn new(dir: String, name: String, server: String, db: String, orm: String) -> Self {
        Self {
            dir,
            name,
            server,
            db,
            orm,
        }
    }

    pub fn create_project(&self) -> io::Result<()> {
        let project_dir = Path::new(&self.dir).join(&self.name);

        // Run `cargo new` to initialize the project
        let cargo_new = Command::new("cargo")
            .args(["new", &self.name, "--bin"])
            .current_dir(&self.dir)
            .status()?;
        if !cargo_new.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to run `cargo new`",
            ));
        }

        // server installation
        match self.server.as_str() {
            "axum" => {
                // Install dependencies
                install_dependency(&project_dir, &self.server, None, None)?;
            }
            "actix-web" => install_dependency(&project_dir, &self.server, None, None)?,
        }
        // necessory thing
        install_dependency(&project_dir, "tokio", Some("1.0"), Some(vec!["full"]))?;

        // orm installation
        match self.orm.as_str() {
            "sqlx" => {
                install_dependency(
                    &project_dir,
                    &self.orm,
                    Some(if self.orm == "sqlx" { "0.7" } else { "2.0" }),
                    Some(vec!["runtime-tokio-rustls", &self.db]),
                )?;
            }
        }

        install_dependency(&project_dir, "jsonwebtoken", Some("8.3"), None)?;
        install_dependency(&project_dir, "serde", Some("1.0"), Some(vec!["derive"]))?;
        install_dependency(&project_dir, "figment", Some("0.10"), Some(vec!["env"]))?;
        install_dependency(&project_dir, "reqwest", Some("0.11"), Some(vec!["json"]))?;

        // Create modular folder structure
        create_dir_all(project_dir.join("src/routes"))?;
        create_dir_all(project_dir.join("src/models"))?;
        create_dir_all(project_dir.join("src/config"))?;

        // Write Rust files using procedural macros
        let main_code = match self.server.as_str() {
            "axum" => {
                let main = generate_axum_main!();
                quote::quote!(#main).to_string()
            }
            "actix-web" => {
                let main = generate_actix_main!(self.name);
                quote::quote!(#main).to_string()
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported server framework",
                ))
            }
        };

        fs::write(project_dir.join("src/main.rs"), main_code)?;

        let routes_code = match self.server.as_str() {
            "axum" => {
                let routes = generate_axum_routes!();
                quote::quote!(#routes).to_string()
            }
            "actix-web" => {
                let routes = generate_actix_routes!();
                quote::quote!(#routes).to_string()
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported server framework",
                ))
            }
        };
        fs::write(project_dir.join("src/routes/example.rs"), routes_code)?;

        fs::write(project_dir.join("README.md"), readme::TEMPLATE)?;
        fs::write(project_dir.join(".env"), env::TEMPLATE)?;

        Ok(())
    }
}
