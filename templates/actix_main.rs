use actix_web::{get, App, HttpServer, Responder};
mod routes;

#[get("/")]
async fn hello() -> impl Responder {
    concat!("Hello, World ", "!")
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
