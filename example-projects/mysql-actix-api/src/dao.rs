use crate::db::Pool;
use crate::errors::ServiceError;
use crate::models::User;

use crate::schema::user::dsl::*;
use diesel::prelude::*;

pub fn find_user_by_user_id(pool: &Pool, user_id: i32) -> Result<User, ServiceError> {
    let conn = pool.get()?;

    user.filter(id.eq(user_id))
        .get_result::<User>(&conn)
        .map_err(|err| ServiceError::BadRequest(err.to_string()))
}
