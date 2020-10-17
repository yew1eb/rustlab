use crate::handlers::*;
use actix_web::web;

pub fn url_config(cfg: &mut web::ServiceConfig) {
    index_urls_config(cfg);
    user_urls_config(cfg);
}

fn index_urls_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .route("", web::get().to(index))
            .route("status", web::get().to(status)),
    );
}

fn user_urls_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/user/{user_id}", web::get().to(get_user))
            //.route("/user", web::post().to(add_user))
            //.route("/users", web::get().to(get_users)),
    );
}
