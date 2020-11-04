use actix_web::{web, HttpResponse};

use crate::dao;
use crate::errors::ServiceError;
use crate::{db::Pool, helpers::respond_json};

pub async fn get_user(
    pool: web::Data<Pool>,
    user: web::Path<i32>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = user.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || dao::find_user_by_user_id(&pool, user_id)).await?;

    respond_json(user)
}
