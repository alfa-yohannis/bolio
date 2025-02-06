use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;

// Template for the index page
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
    pub username: String,           // Use Option to handle absence of a username
}

pub async fn index(req: HttpRequest) -> impl Responder {
    // Retrieve session_id and username from cookies
    let session_id = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());
    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    let template = IndexTemplate {
        session_id: session_id.clone(),
        username: username.unwrap_or_else(|| "Guest".to_string()),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}






