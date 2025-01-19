use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use crate::{models::NewUser, schema::users};
use crate::DbPool;

pub async fn handle_signup(
    pool: web::Data<DbPool>,
    form: web::Form<NewUser>, // Use NewUser without lifetimes
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    // Hash the password
    let hashed_password = hash(&form.password, DEFAULT_COST).unwrap();

    let new_user = NewUser {
        username: form.username.clone(),
        email: form.email.clone(),
        password: hashed_password,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok().body("Signup successful"),
        Err(_) => HttpResponse::InternalServerError().body("Error signing up"),
    }
}
