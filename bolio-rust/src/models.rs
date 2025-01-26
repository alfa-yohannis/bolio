use crate::schema::{conversion_transactions, credit_transactions, users};
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

/// Represents a user in the database
#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub credit: i64, // Credit is stored in bytes (BIGINT)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub session_id: Option<String>,
}

/// Used for inserting or updating user data
#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub last_login: Option<DateTime<Utc>>,
    pub session_id: Option<String>,
}

/// Represents a credit transaction in the database
#[derive(Queryable, Debug, Serialize)]
pub struct CreditTransaction {
    pub id: i32,
    pub user_id: i32, // Foreign key to `users`
    pub transaction_type: String,
    pub amount: i64, // Credit amount in bytes (BIGINT)
    pub source: String,
    pub transaction_id: String, // Unique identifier for the transaction
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub description: Option<String>, // Optional description
}

/// Used for inserting or updating credit transaction data
#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "credit_transactions"]
pub struct NewCreditTransaction {
    pub user_id: i32,
    pub transaction_type: String,
    pub amount: i64,
    pub source: String,
    pub transaction_id: String,
    pub status: String,
    pub description: Option<String>,
}

/// Represents a file conversion transaction in the database
#[derive(Queryable, Debug, Serialize)]
pub struct ConversionTransaction {
    pub id: i32,
    pub user_id: i32, // Foreign key to `users`
    pub source_size: i64, // Size of the source file in bytes (BIGINT)
    pub target_size: i64, // Size of the target file in bytes (BIGINT)
    pub credit_used: i64, // Total credit used (BIGINT)
    pub conversion_type: String, // Type of transformation (e.g., "video-to-audio")
    pub source_type: String, // Source file type (e.g., "mp4")
    pub target_type: String, // Target file type (e.g., "txt")
    pub created_at: DateTime<Utc>,
}

/// Used for inserting or updating file conversion transaction data
#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name = "conversion_transactions"]
pub struct NewConversionTransaction {
    pub user_id: i32,
    pub source_size: i64,
    pub target_size: i64,
    pub conversion_type: String,
    pub source_type: String,
    pub target_type: String,
}


// use crate::schema::users;
// use chrono::{DateTime, Utc};
// use diesel::{AsChangeset, Insertable, Queryable};
// use serde::{Deserialize, Serialize}; // Import both Deserialize and Serialize

// #[derive(Queryable, Debug, Serialize)] // Add Serialize for output use cases
// pub struct User {
//     pub id: i32,
//     pub username: String,
//     pub email: String,
//     pub password: String,
//     pub created_at: DateTime<Utc>, // Use DateTime<Utc> for timezone-aware timestamps
//     pub updated_at: DateTime<Utc>, // Use DateTime<Utc> for timezone-aware timestamps
//     pub last_login: Option<DateTime<Utc>>, // Optional field for the last login timestamp
//     pub session_id: Option<String>,       // Optional field for the session ID
// }

// #[derive(Insertable, AsChangeset, Deserialize, Debug)]
// #[table_name = "users"]
// pub struct NewUser {
//     pub username: String, // Use String for ownership handling
//     pub email: String,
//     pub password: String,
//     pub last_login: Option<DateTime<Utc>>, // Allow initializing the last login
//     pub session_id: Option<String>,       // Allow initializing the session ID
// }

