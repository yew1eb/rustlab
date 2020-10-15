use r2d2;
use r2d2_sqlite::SqliteConnectionManager;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub fn create_db_pool(db_url: &str) -> Pool {
    // r2d2 pool
    let manager = SqliteConnectionManager::file("test.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    pool
}
