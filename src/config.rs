use crate::{
    crates::{
        actix::{write_actix, write_actix_routes},
        axum::{write_axum, write_axum_routes},
    },
    utils::{write_env, write_readme},
};
use std::{
    fs::{self, create_dir_all},
    path::Path,
    process::Command,
};

#[derive(Debug)]
pub struct Config {
    pub dir: String,
    pub name: String,
    pub server: String,
    pub db: String,
    pub orm: String,
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

    pub fn create_project(&self) -> std::io::Result<()> {
        let _project_dir = Path::new(&self.dir);

        // Run `cargo new` to initialize the project
        let cargo_new = Command::new("cargo")
            .args(["new", &self.name, "--bin"])
            .current_dir(&self.dir)
            .status()?;
        if !cargo_new.success() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to run `cargo new`",
            ));
        }

        // Install dependencies using `cargo add`
        let project_path = Path::new(&self.dir).join(&self.name);
        install_dependency(
            &project_path,
            &self.server,
            Some(if self.server == "axum" { "0.7" } else { "4.0" }),
            None,
        )?;
        install_dependency(&project_path, "tokio", Some("1.0"), Some(vec!["full"]))?;
        install_dependency(
            &project_path,
            &self.orm,
            Some(if self.orm == "sqlx" { "0.7" } else { "2.0" }),
            Some(vec!["runtime-tokio-rustls", &self.db]),
        )?;
        install_dependency(&project_path, "jsonwebtoken", Some("8.3"), None)?;
        install_dependency(&project_path, "serde", Some("1.0"), Some(vec!["derive"]))?;
        install_dependency(&project_path, "figment", Some("0.10"), Some(vec!["env"]))?;
        install_dependency(&project_path, "reqwest", Some("0.11"), Some(vec!["json"]))?;

        // Create src/main.rs with a server-specific implementation
        let main_rs = match self.server.as_str() {
            "axum" => write_axum(),
            "actix-web" => write_actix(),
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unsupported server framework",
                ))
            }
        };
        fs::write(project_path.join("src/main.rs"), main_rs)?;

        // Create modular folder structure
        create_dir_all(project_path.join("src/routes"))?;
        create_dir_all(project_path.join("src/models"))?;
        create_dir_all(project_path.join("src/config"))?;

        // Create a basic route example
        let route_example = match self.server.as_str() {
            "axum" => write_axum_routes(),
            "actix-web" => write_actix_routes(),
            _ => "",
        };
        fs::write(project_path.join("src/routes/mod.rs"), route_example)?;

        // Create .env file
        let env = write_env(&self.db, &self.name);
        fs::write(project_path.join(".env"), env)?;

        // Create README.md
        let readme = write_readme(&self.name, &self.db, &self.server, &self.orm);
        fs::write(project_path.join("README.md"), readme)?;
        Ok(())
    }
}

fn install_dependency(
    project_dir: &Path,
    dep: &str,
    version: Option<&str>,
    features: Option<Vec<&str>>,
) -> std::io::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("add").current_dir(project_dir);

    match version {
        Some(ver) => cmd.arg(format!("{}@{}", dep, ver)),
        None => cmd.arg(dep),
    };

    if let Some(feat) = features {
        if !feat.is_empty() {
            cmd.arg(format!("--features={}", feat.join(",")));
        }
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to add dependency: {}", dep),
        ));
    }

    Ok(())
}
