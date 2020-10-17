pub use chrono::{Local, NaiveDateTime};
pub use diesel::prelude::*;
use diesel::*;
pub use uuid::Uuid;

use crate::schema::*;

// This represents a user pulled from then database
#[derive(Queryable, PartialEq, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_type: i8,
    pub create_time: NaiveDateTime,
}

// This represents a user being inserted into the database
#[derive(Clone, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_type: i8,
    pub create_time: NaiveDateTime,
}

// This represents a user being updated into the database
#[derive(Clone, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub role_type: i8,
    pub create_time: NaiveDateTime,
}