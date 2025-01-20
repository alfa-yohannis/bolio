use actix_web::{HttpResponse, Responder};
use actix_web::http::header::{LOCATION, SET_COOKIE};

pub async fn signout_handler() -> impl Responder {
    HttpResponse::SeeOther()
        .append_header((LOCATION, "/")) // Redirect to the index page
        .append_header((
            SET_COOKIE,
            "session_id=; HttpOnly; Path=/; Max-Age=0", // Clear the session cookie
        ))
        .finish()
}
