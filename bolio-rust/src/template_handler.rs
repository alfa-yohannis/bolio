use actix_web::{HttpResponse, Responder};
use askama::Template;

// Template for the index page
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

pub async fn index() -> impl Responder {
    let template = IndexTemplate;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Template for the signup page
#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate;

pub async fn signup() -> impl Responder {
    let template = SignupTemplate;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Template for the signin page
#[derive(Template)]
#[template(path = "signin.html")]
pub struct SigninTemplate;

pub async fn signin() -> impl Responder {
    let template = SigninTemplate;

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}
