use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn create_db_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url));
    pool
}
