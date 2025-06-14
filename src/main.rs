use clap::Parser;
use clap_derive::{Parser, Subcommand};
use std::io::{self, Write};
mod config;
use config::Config;
mod crates;
mod utils;

#[derive(Parser, Debug)]
#[clap(
    name = "rustack",
    version = "0.1.0",
    about = "A CLI tool to scaffold Rust backend projects"
)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new Rust backend project with interactive configuration
    Create {
        /// Optional project name
        #[clap(default_value = "my-rustack-app")]
        name: String,
    },
    /// Create a new Rust backend project with predefined configuration
    New {
        /// Project name
        name: String,
        /// Database type (e.g., postgres, mysql)
        #[clap(long, default_value = "postgres")]
        db: String,
        /// ORM type (e.g., sqlx, diesel)
        #[clap(long, default_value = "sqlx")]
        orm: String,
        /// Server framework (e.g., axum, actix-web)
        #[clap(long, default_value = "axum")]
        server: String,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create { name } => {
            // Interactive mode
            println!("Welcome to rustack! Let's configure your project.");
            let mut config = Config::new(
                String::from("."),
                if name.is_empty() {
                    String::from("my-rustack-app")
                } else {
                    name
                },
                String::from("axum"),
                String::from("postgres"),
                String::from("sqlx"),
            );

            // Interactive configuration loop
            loop {
                println!("\nCurrent configuration:");
                println!("  Project name: {}", config.name);
                println!("  Server framework: {}", config.server);
                println!("  Database: {}", config.db);
                println!("  ORM: {}", config.orm);
                println!("\nOptions:");
                println!("  1. Change project name");
                println!("  2. Change server framework (current: {})", config.server);
                println!("  3. Change database (current: {})", config.db);
                println!("  4. Change ORM (current: {})", config.orm);
                println!("  5. Create project with current configuration");
                println!("  6. Exit without creating");

                print!("\nEnter option (1-6): ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let choice = input.trim();

                match choice {
                    "1" => {
                        print!("Enter project name (current: {}): ", config.name);
                        io::stdout().flush()?;
                        let mut name = String::new();
                        io::stdin().read_line(&mut name)?;
                        let name = name.trim();
                        if !name.is_empty() {
                            config.name = name.to_string();
                        }
                    }
                    "2" => {
                        print!("Enter server framework (axum/actix-web, default: axum): ");
                        io::stdout().flush()?;
                        let mut server = String::new();
                        io::stdin().read_line(&mut server)?;
                        let server = server.trim();
                        if !server.is_empty() && (server == "axum" || server == "actix-web") {
                            config.server = server.to_string();
                        } else if !server.is_empty() {
                            println!("Invalid server framework. Using default: axum");
                        }
                    }
                    "3" => {
                        print!("Enter database (postgres/mysql, default: postgres): ");
                        io::stdout().flush()?;
                        let mut db = String::new();
                        io::stdin().read_line(&mut db)?;
                        let db = db.trim();
                        if !db.is_empty() && (db == "postgres" || db == "mysql") {
                            config.db = db.to_string();
                        } else if !db.is_empty() {
                            println!("Invalid database. Using default: postgres");
                        }
                    }
                    "4" => {
                        print!("Enter ORM (sqlx/diesel, default: sqlx): ");
                        io::stdout().flush()?;
                        let mut orm = String::new();
                        io::stdin().read_line(&mut orm)?;
                        let orm = orm.trim();
                        if !orm.is_empty() && (orm == "sqlx" || orm == "diesel") {
                            config.orm = orm.to_string();
                        } else if !orm.is_empty() {
                            println!("Invalid ORM. Using default: sqlx");
                        }
                    }
                    "5" => {
                        config.create_project()?;
                        println!("Created new project: {}", config.name);
                        break;
                    }
                    "6" => {
                        println!("Exiting without creating project.");
                        return Ok(());
                    }
                    _ => println!("Invalid option. Please enter 1-6."),
                }
            }
        }
        Commands::New {
            name,
            db,
            orm,
            server,
        } => {
            // Single-command mode
            let config = Config::new(String::from("."), name, server, db, orm);
            config.create_project()?;
            println!("Created new project: {}", config.name);
        }
    }

    Ok(())
}
