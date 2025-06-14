use clap::Parser;
use clap_derive::{Parser, Subcommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::io;
mod config;
use config::Config;

mod crates;
mod utils;

#[derive(Parser, Debug)]
#[clap(
    name = "rustack",
    version = "0.1.0",
    about = "🚀 Scaffold Rust backend projects easily"
)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Create,
    New {
        name: String,
        #[clap(long, default_value = "postgres")]
        db: String,
        #[clap(long, default_value = "sqlx")]
        orm: String,
        #[clap(long, default_value = "axum")]
        server: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create => {
            println!("{}", "🦀 Welcome to rustack!".bold().green());
            println!("{}", "Let's set up your Rust backend project.\n".dimmed());

            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .default("my-rustack-app".into())
                .interact_text()
                .unwrap();

            let server_options = vec!["axum", "actix-web"];
            let server_index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose a server framework")
                .default(0)
                .items(&server_options)
                .interact()
                .unwrap();
            let server = server_options[server_index].to_string();

            let db_options = vec!["postgres", "mysql"];
            let db_index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose a database")
                .default(0)
                .items(&db_options)
                .interact()
                .unwrap();
            let db = db_options[db_index].to_string();

            let orm_options = vec!["sqlx", "diesel"];
            let orm_index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose an ORM")
                .default(0)
                .items(&orm_options)
                .interact()
                .unwrap();
            let orm = orm_options[orm_index].to_string();

            let config = Config::new(".".into(), name.clone(), server, db, orm);
            config.create_project()?;
            println!("\n{} {}", "✅ Created project:".green().bold(), name);
        }
        Commands::New {
            name,
            db,
            orm,
            server,
        } => {
            let config = Config::new(".".into(), name.clone(), server, db, orm);
            config.create_project()?;
            println!("{} {}", "✅ Created project:".green().bold(), name);
        }
    }

    Ok(())
}
