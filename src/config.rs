use crate::templates::{ACTIX_MAIN, ACTIX_ROUTES, AXUM_MAIN, AXUM_ROUTES};
use crate::{
    templates::{env, readme},
    utils::install_dependency,
};
use colored::Colorize;
use serde::Serialize;
use std::{
    fs::{self, create_dir_all},
    io,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Debug, Serialize)]
pub struct Config {
    name: String,
    server: String,
    db: String,
    orm: String,
}

impl Config {
    pub fn new(name: String, server: String, db: String, orm: String) -> Self {
        Self {
            name,
            server,
            db,
            orm,
        }
    }

    pub fn create_project(&self) -> io::Result<()> {
        println!("Creating new project {}", self.name.purple());

        let root_dir = Path::new(".");
        let project_dir = root_dir.join(&self.name);

        if project_dir.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Project already exists",
            ));
        }

        create_dir_all(&project_dir)?;

        // Check if user has nightly toolchain
        let rustc_output = Command::new("rustc")
            .arg("--version")
            .output()
            .unwrap_or_else(|_| panic!("Rust not installed or not in PATH"));

        let rustc_version = String::from_utf8_lossy(&rustc_output.stdout);
        let use_nightly = rustc_version.contains("nightly");

        let edition = if use_nightly { "2024" } else { "2021" };

        let status = Command::new("cargo")
            .args(["init", "--bin", "--edition", edition])
            .current_dir(&project_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to run `cargo init`",
            ));
        }

        if use_nightly {
            let toolchain_toml = r#"[toolchain]
    channel = "nightly"
    "#;
            fs::write(project_dir.join("rust-toolchain.toml"), toolchain_toml)?;
        } else {
            eprintln!(
                "{}",
                "⚠️  Nightly Rust not detected — falling back to edition 2021"
                    .yellow()
                    .bold()
            );
            eprintln!("   To enable edition 2024 support, run: rustup override set nightly\n");
        }

        println!(
            "{}",
            "📦 Installing crates... (this may take a moment)"
                .green()
                .bold()
        );

        match self.server.as_str() {
            "axum" => install_dependency(&project_dir, &self.server, Some("0.8.4"), None)?,
            "actix-web" => install_dependency(&project_dir, &self.server, None, None)?,
            _ => {}
        }

        install_dependency(&project_dir, "tokio", Some("1.0"), Some(vec!["full"]))?;

        if self.orm == "sqlx" {
            install_dependency(
                &project_dir,
                "sqlx",
                Some("0.7"),
                Some(vec!["runtime-tokio-rustls", &self.db]),
            )?;
        }

        install_dependency(&project_dir, "jsonwebtoken", Some("8.3"), None)?;
        install_dependency(&project_dir, "serde", Some("1.0"), Some(vec!["derive"]))?;
        install_dependency(&project_dir, "figment", Some("0.10"), Some(vec!["env"]))?;
        install_dependency(&project_dir, "reqwest", Some("0.11"), Some(vec!["json"]))?;

        create_dir_all(project_dir.join("src/routes"))?;
        create_dir_all(project_dir.join("src/models"))?;
        create_dir_all(project_dir.join("src/config"))?;

        let main_code = match self.server.as_str() {
            "axum" => AXUM_MAIN.to_string(),
            "actix-web" => ACTIX_MAIN.to_string(),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unsupported server framework",
                ))
            }
        };
        fs::write(project_dir.join("src/main.rs"), main_code)?;

        let routes_code = match self.server.as_str() {
            "axum" => AXUM_ROUTES.to_string(),
            "actix-web" => ACTIX_ROUTES.to_string(),
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
