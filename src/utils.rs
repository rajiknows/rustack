//! This module contains utility functions for generating README and .env files.
use std::{
    io,
    path::Path,
    process::{Command, Stdio},
};

pub fn install_dependency(
    project_dir: &Path,
    dep: &str,
    version: Option<&str>,
    features: Option<Vec<&str>>,
) -> io::Result<()> {
    let mut cmd = Command::new("cargo");

    cmd.arg("add").current_dir(project_dir);
    if let Some(ver) = version {
        cmd.arg(format!("{}@{}", dep, ver));
    } else {
        cmd.arg(dep);
    }

    if let Some(feat) = features {
        if !feat.is_empty() {
            cmd.arg(format!("--features={}", feat.join(",")));
        }
    }

    // 🔇 silence stdout + stderr
    // cmd.stdout(Stdio::null()).stderr(Stdio::null());

    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to add dependency: {}", dep),
        ));
    }

    Ok(())
}
