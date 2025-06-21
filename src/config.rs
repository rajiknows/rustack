use crate::templates::{ACTIX_MAIN, ACTIX_ROUTES, AXUM_MAIN, AXUM_ROUTES};
use crate::utils::{has_nightly_installed, install_nightly_toolchain, write_rust_toolchain_file};
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

        let mut use_nightly = false;
        let is_axum_nightly = self.server == "axum-nightly";

        if is_axum_nightly {
            use_nightly = has_nightly_installed();
            if !use_nightly {
                println!("{}", "🔧 Installing nightly toolchain...".blue());
                install_nightly_toolchain()?;
                use_nightly = true;
            }
        }

        let edition = if use_nightly { "2024" } else { "2021" };

        let status = Command::new("cargo")
            .args(["init", "--bin", "--edition", edition, &self.name])
            .current_dir(root_dir)
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
            write_rust_toolchain_file(&project_dir)?;
        } else if is_axum_nightly {
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
            "axum-nightly" | "axum" => {
                let axum_version = if use_nightly { "0.8.4" } else { "0.7.5" };
                install_dependency(&project_dir, "axum", Some(axum_version), None)?;
            }
            "actix-web" => {
                install_dependency(&project_dir, "actix-web", None, None)?;
            }
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
            "axum" | "axum-nightly" => AXUM_MAIN.to_string(),
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
            "axum" | "axum-nightly" => AXUM_ROUTES.to_string(),
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
