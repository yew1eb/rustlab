use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}


#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("ok!")
}


pub async fn empty() -> impl Responder {
    HttpResponse::Ok().body("empty!")
}