use actix_web::{middleware, get, App, HttpRequest, HttpResponse, HttpServer, Responder};


pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}


pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}