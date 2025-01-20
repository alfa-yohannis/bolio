use crate::DbPool;
use crate::{models::NewUser, schema::users};
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

pub async fn handle_signup(
    pool: web::Data<DbPool>,
    form: web::Form<NewUser>, // Use NewUser without lifetimes
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    // Hash the password
    let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();

    // Create a new user with optional fields (last_login and session_id)
    let new_user = NewUser {
        username: form.username.clone(),
        email: form.email.clone(),
        password: hashed_password,
        last_login: None, // Initialize as None for new signups
        session_id: None, // Initialize as None for new signups
    };

    // Insert the new user into the database
    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().body("Signup successful"),
        Err(e) => {
            eprintln!("Error inserting new user: {:?}", e); // Print the error to stderr
            HttpResponse::InternalServerError().body("Error signing up")
        }
    }
}
