use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use serde::Deserialize; // Import serde::Deserialize

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)] // Add Deserialize here
#[table_name = "users"]
pub struct NewUser {
    pub username: String,       // Use String instead of &str
    pub email: String,          // Use String instead of &str
    pub password: String,  // Use String instead of &str
}
