use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize}; // Import both Deserialize and Serialize

#[derive(Queryable, Debug, Serialize)] // Add Serialize for output use cases
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>, // Use DateTime<Utc> for timezone-aware timestamps
    pub updated_at: DateTime<Utc>, // Use DateTime<Utc> for timezone-aware timestamps
    pub last_login: Option<DateTime<Utc>>, // Optional field for the last login timestamp
    pub session_id: Option<String>,       // Optional field for the session ID
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String, // Use String for ownership handling
    pub email: String,
    pub password: String,
    pub last_login: Option<DateTime<Utc>>, // Allow initializing the last login
    pub session_id: Option<String>,       // Allow initializing the session ID
}

// use crate::schema::users;
// use chrono::NaiveDateTime;
// use diesel::{AsChangeset, Insertable, Queryable};
// use serde::Deserialize; // Import serde::Deserialize

// #[derive(Queryable, Debug)]
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub email: String,
//     pub password: String,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }

// #[derive(Insertable, AsChangeset, Deserialize, Debug)] // Add Deserialize here
// #[table_name = "users"]
// pub struct NewUser {
//     pub username: String,       // Use String instead of &str
//     pub email: String,          // Use String instead of &str
//     pub password: String,  // Use String instead of &str
// }
