use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn generate_axum_main(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as syn::LitStr).value();
    let code = quote! {
        use axum::{routing::get, Router};
        use tokio::net::TcpListener;
        mod routes;

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            let app = Router::new()
                .route("/", get(|| async { concat!("Hello, ", #name, "!") }))
                .nest("/api", routes::example::router());

            let listener = TcpListener::bind("0.0.0.0:3000").await?;
            axum::serve(listener, app).await?;

            Ok(())
        }
    };
    TokenStream::from(code)
}

#[proc_macro]
pub fn generate_axum_routes(_input: TokenStream) -> TokenStream {
    let code = quote! {
        use axum::{routing::get, Router};

        pub fn router() -> Router {
            Router::new().route("/example", get(example_handler))
        }

        async fn example_handler() -> &'static str {
            "This is an example route!"
        }
    };
    TokenStream::from(code)
}

#[proc_macro]
pub fn generate_actix_main(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as syn::LitStr).value();
    let code = quote! {
        use actix_web::{get, App, HttpServer, Responder};
        mod routes;

        #[get("/")]
        async fn hello() -> impl Responder {
            concat!("Hello, ", #name, "!")
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
    };
    TokenStream::from(code)
}

#[proc_macro]
pub fn generate_actix_routes(_input: TokenStream) -> TokenStream {
    let code = quote! {
        use actix_web::{get, HttpResponse, Responder};

        #[get("/example")]
        pub async fn example_handler() -> impl Responder {
            HttpResponse::Ok().body("This is an example route!")
        }
    };
    TokenStream::from(code)
}
