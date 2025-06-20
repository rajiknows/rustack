use actix_web::{get, HttpResponse, Responder};

#[get("/example")]
pub async fn example_handler() -> impl Responder {
    HttpResponse::Ok().body("This is an example route!")
}
