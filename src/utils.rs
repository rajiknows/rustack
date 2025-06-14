//! This module contains utility functions for generating README and .env files.
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
