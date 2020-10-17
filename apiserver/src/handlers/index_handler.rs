use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
