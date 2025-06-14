//! Crate for Actix Web server

pub fn write_actix() -> &'static str {
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

pub fn write_actix_routes() -> &'static str {
    r#"use actix_web::{get, HttpResponse, Responder};

#[get("/example")]
pub async fn example_handler() -> impl Responder {
HttpResponse::Ok().body("This is an example route!")
}
"#
}
