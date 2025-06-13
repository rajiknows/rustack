use clap::Parser;
use clap_derive::{Parser, Subcommand};
use std::{
    fs::{self, create_dir_all},
    io::{self, Write},
    path::Path,
    process::Command,
};

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

#[derive(Debug)]
struct Config {
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
            "axum" => {
                r#"use axum::{routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(|| async { "Hello, Rustack!" }));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
"#
            }
            "actix-web" => {
                r#"use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Rustack!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(hello)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
"#
            }
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
            "axum" => {
                r#"use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/example", get(example_handler))
}

async fn example_handler() -> &'static str {
    "This is an example route!"
}
"#
            }
            "actix-web" => {
                r#"use actix_web::{get, HttpResponse, Responder};

#[get("/example")]
pub async fn example_handler() -> impl Responder {
    HttpResponse::Ok().body("This is an example route!")
}
"#
            }
            _ => "",
        };
        fs::write(project_path.join("src/routes/mod.rs"), route_example)?;

        // Update main.rs to include routes
        let main_rs_with_routes = match self.server.as_str() {
            "axum" => {
                r#"use axum::{routing::get, Router};
use tokio::net::TcpListener;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Rustack!" }))
        .nest("/api", routes::example::router());

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
"#
            }
            "actix-web" => {
                r#"use actix_web::{get, App, HttpServer, Responder};
mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Rustack!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(routes::example::example_handler)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
"#
            }
            _ => "",
        };
        fs::write(project_path.join("src/main.rs"), main_rs_with_routes)?;

        // Create .env file
        let env = format!(
            r#"DATABASE_URL={}://user:password@localhost:5432/{}"#,
            self.db, self.name
        );
        fs::write(project_path.join(".env"), env)?;

        // Create README.md
        let readme = format!(
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
            self.name, self.db, self.server, self.orm
        );
        fs::write(project_path.join("README.md"), readme)?;

        Ok(())
    }
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
