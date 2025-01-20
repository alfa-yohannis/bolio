use actix_web::{web, HttpResponse, Responder};
use actix_web::http::header::{LOCATION, SET_COOKIE};
use bcrypt::verify;
use diesel::prelude::*;
use serde::Deserialize;
use crate::{models::User, schema::users::dsl::*, DbPool};
use chrono::Utc;
use uuid::Uuid;

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
    let user: Result<User, _> = users
        .filter(username.eq(&form.username))
        .first(&mut conn);

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

                // Redirect to index with a session cookie
                HttpResponse::SeeOther()
                    .append_header((LOCATION, "/")) // Redirect to the index page
                    .append_header((
                        SET_COOKIE,
                        format!("session_id={}; Path=/", new_session_id), // Set session cookie
                    ))
                    .finish()
            } else {
                HttpResponse::Unauthorized().body("Invalid username or password")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}