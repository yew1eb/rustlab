use actix_web::{web, HttpResponse};
use failure::Error;


use crate::db::Pool;
use crate::models::User;

pub async fn get_user(
    pool: web::Data<Pool>,
    user: web::Path<i64>,
) -> Result<HttpResponse, Error>{
    let user_id = user.into_inner();
    let conn = pool.get().unwrap();
    
    let uuid = format!("{}", uuid::Uuid:new_v4());
    conn.execute(
        "INSERT INTO user (id, name) VALUES ($1, $2)",
        &[&uuid, &user_id.to_string()],
    )
    .unwrap();


    let user = conn.query_row("SELECT name FROM user WHERE id=$1", &[&uuid], |row| {
        row.get::<_, String>(0)
    })?;

    Ok(HttpResponse::Ok().json(user))
}
