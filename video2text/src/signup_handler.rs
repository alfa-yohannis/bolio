use crate::DbPool;
use crate::{models::NewUser, schema::users};
use actix_web::http::header::{LOCATION, SET_COOKIE};
use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;

use actix_web::HttpRequest;
use askama::Template;

// Template for the signup page
#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
    pub username: String,           // Use Option to handle absence of a username
}

pub async fn signup(req: HttpRequest) -> impl Responder {
    // Retrieve session_id and username from cookies
    let session_id = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());
    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    let template = SignupTemplate {
        session_id,
        username: username.unwrap_or_else(|| "Guest".to_string()),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

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
        Ok(_) => HttpResponse::SeeOther()
            .append_header((LOCATION, "/signin")) // Redirect to the index page
            .append_header((
                SET_COOKIE,
                "session_id=; HttpOnly; Path=/; Max-Age=0", // Clear the session cookie
            ))
            .append_header((
                SET_COOKIE,
                "username=; Path=/; Max-Age=0", // Clear the username cookie
            ))
            .finish(),
        Err(e) => {
            eprintln!("Error inserting new user: {:?}", e); // Print the error to stderr
            HttpResponse::InternalServerError().body("Error signing up")
        }
    }
}
