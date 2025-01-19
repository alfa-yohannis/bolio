use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

// Struct to capture signup form data
#[derive(Deserialize)]
pub struct SignupData {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

// Route to render the signup page
pub async fn render_signup_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../templates/signup.html"))
}

// Route to handle signup form submission
pub async fn handle_signup(form: web::Form<SignupData>) -> impl Responder {
    if form.password != form.confirm_password {
        return HttpResponse::BadRequest().body("Passwords do not match.");
    }

    if !form.email.contains('@') {
        return HttpResponse::BadRequest().body("Invalid email address.");
    }

    // Here you would typically save the user data to a database.
    println!(
        "New user signed up: username={}, email={}",
        form.username, form.email
    );

    HttpResponse::Ok().body("Signup successful!")
}
