pub mod readme {
    pub const TEMPLATE: &str = r#"# {{ name }}

A Rust backend project scaffolded with rustack.

## Setup
1. Ensure you have Rust installed: `rustup update`
2. Set up your {{ db }} database and update `.env` with the correct DATABASE_URL.
3. Run the project: `cargo run`

## Features
- {{ server }} web framework
- {{ orm }} for database interactions
- JWT authentication
- Modular project structure (src/routes, src/models, src/config)

## Contributing
This is an open-source project! Feel free to contribute on GitHub.
"#;
}

pub mod env {
    pub const TEMPLATE: &str = r#"DATABASE_URL={{ db }}://user:password@localhost:5432/{{ name }}"#;
}

pub const AXUM_MAIN: &str = include_str!("../templates/axum_main.rs");
pub const AXUM_ROUTES: &str = include_str!("../templates/axum_routes.rs");
pub const ACTIX_MAIN: &str = include_str!("../templates/actix_main.rs");
pub const ACTIX_ROUTES: &str = include_str!("../templates/actix_routes.rs");
pub const CONFIG_FILE: &str = include_str!("../templates/config.rs");
