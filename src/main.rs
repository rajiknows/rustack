use std::{fs::create_dir, path::Path, process::Command};

use clap::{self, Parser};

#[derive(Parser, Debug)]
#[clap(
    name = "rustack",
    version = "0.1.0",
    about = "A cli tool to scaffold rust backend"
)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

/// the Command line arguments
enum Commands {
    /// Create a new Rust Backend Project
    New {
        /// Project name
        name: String,

        #[clap(long, default_value = "postgres")]
        db: String,

        #[clap(long, default_value = "sqlx")]
        orm: String,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::New { name, db, orm } => {
            create_project(&name, db, auth);
            println!("Created new Project {}", name);
        }
    }

    Ok(())
}

fn create_project(name: &String, db: String, orm: String) {
    let project_dir = Path::new(name);
    create_dir(path);

    // now we need to run some commands here .....
    // we will not do manualt file cretion atleast at this step
    println!("cargo new {}", name);
    let cargo_new = Command::new("cargo").args(["new", name, "--bin"]).status();
    if !cargo_new.ok() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to run cargo new",
        ));
    }
}
