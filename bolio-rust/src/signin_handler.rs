// use crate::{models::User, schema::users::dsl::*, DbPool};
// use actix_web::http::header::{LOCATION, SET_COOKIE};
// use actix_web::{web, HttpRequest, HttpResponse, Responder};
// use askama::Template;
// use bcrypt::verify;
// use chrono::Utc;
// use diesel::prelude::*;
// use serde::Deserialize;
// use uuid::Uuid;

use crate::models::User;
use crate::schema::users::dsl::*;
use crate::DbPool;
use actix_web::http::header::{LOCATION, SET_COOKIE};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama::Template;
use bcrypt::verify;
use chrono::Utc;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;

// Template for the signin page
#[derive(Template)]
#[template(path = "signin.html")]
pub struct SigninTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
    pub username: String,           // Use Option to handle absence of a username
}

pub async fn signin(req: HttpRequest) -> impl Responder {
    // Retrieve session_id and username from cookies
    let session_cookie = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());
    let username_cookie = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    let template = SigninTemplate {
        session_id: session_cookie,
        username: username_cookie.unwrap_or_else(|| "Guest".to_string()),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

#[derive(Deserialize)]
pub struct SignInForm {
    pub username: String,
    pub password: String,
}

pub async fn signin_handler(
    pool: web::Data<DbPool>,
    form: web::Form<SignInForm>,
) -> impl Responder {
    let mut conn = pool.get().expect("Couldn't get DB connection from pool");

    // Retrieve the user by username
    let user: Result<User, _> = users.filter(username.eq(&form.username)).first(&mut conn);

    match user {
        Ok(user) => {
            // Verify the password
            if verify(&form.password, &user.password).unwrap_or(false) {
                // Generate a new session ID
                let new_session_id = Uuid::new_v4().to_string();
                let login_time = Utc::now();

                // Update last_login and session_id in the database
                if let Err(e) = diesel::update(users.find(user.id))
                    .set((
                        last_login.eq(Some(login_time)),
                        session_id.eq(Some(new_session_id.clone())),
                    ))
                    .execute(&mut conn)
                {
                    eprintln!("Error updating user session: {:?}", e);
                    return HttpResponse::InternalServerError().body("Error updating session");
                }

                // Redirect to index with session cookies
                HttpResponse::SeeOther()
                    .append_header((LOCATION, "/")) // Redirect to the index page
                    .append_header((
                        SET_COOKIE,
                        format!("session_id={}; Path=/; HttpOnly", new_session_id), // Session ID cookie
                    ))
                    .append_header((
                        SET_COOKIE,
                        format!("username={}; Path=/", user.username), // Username cookie
                    ))
                    .finish()
            } else {
                HttpResponse::Unauthorized().body("Invalid username or password")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}
