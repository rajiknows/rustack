//! This module contains utility functions for generating README and .env files.
use std::{io, path::Path, process::Command};
use tera::{Context, Tera};

pub fn render_template(template: &str, context: &Context) -> io::Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("template", template)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    tera.render("template", context)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn install_dependency(
    project_dir: &Path,
    dep: &str,
    version: Option<&str>,
    features: Option<Vec<&str>>,
) -> io::Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("add").arg(dep).current_dir(project_dir);

    if let Some(ver) = version {
        cmd.arg(format!("--vers={}", ver));
    }

    if let Some(feat) = features {
        if !feat.is_empty() {
            cmd.arg(format!("--features={}", feat.join(",")));
        }
    }

    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to add dependency: {}", dep),
        ));
    }

    Ok(())
}

pub fn write_readme(name: &str, db: &str, server: &str, orm: &str) -> String {
    format!(
        r#"# {}

   A Rust backend project scaffolded with rustack.

   ## Setup
   1. Ensure you have Rust installed: `rustup update`
   2. Set up your {} database and update `.env` with the correct DATABASE_URL.
   3. Run the project: `cargo run`

   ## Features
   - {} web framework
   - {} for database interactions
   - JWT authentication
   - Modular project structure (src/routes, src/models, src/config)

   ## Contributing
   This is an open-source project! Feel free to contribute on GitHub.
   "#,
        name, db, server, orm
    )
}

pub fn write_env(db: &str, name: &str) -> String {
    format!(
        r#"DATABASE_URL={}://user:password@localhost:5432/{}"#,
        db, name
    )
}
