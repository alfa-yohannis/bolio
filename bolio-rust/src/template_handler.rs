use actix_web::{HttpResponse, Responder};
use askama::Template;

// Template for the index page
#[derive(Template)]  
#[template(path = "index.html")] 
pub struct IndexTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
}

pub async fn index(session_id: Option<String>) -> impl Responder {
    let template = IndexTemplate {
        session_id: session_id.clone(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Template for the signup page
#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
}

pub async fn signup(session_id: Option<String>) -> impl Responder {
    let template = SignupTemplate {
        session_id: session_id.clone(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Template for the signin page
#[derive(Template)]
#[template(path = "signin.html")]
pub struct SigninTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
}

pub async fn signin(session_id: Option<String>) -> impl Responder {
    let template = SigninTemplate {
        session_id: session_id.clone(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}


// use actix_web::{HttpResponse, Responder};
// use askama::Template;

// // Template for the index page 
// #[derive(Template)]
// #[template(path = "index.html")] 
// pub struct IndexTemplate {
//     pub session_id: Option<String>, // Use Option to handle absence of a session
// }
 
// pub async fn index() -> impl Responder {
//     let template: IndexTemplate = IndexTemplate {
//         session_id: Some("example_session_id".to_string()), // Example session ID
//     };

//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(template.render().unwrap())
// }

// // Template for the signup page
// #[derive(Template)]    
// #[template(path = "signup.html")] 
// pub struct SignupTemplate {
//     pub session_id: Option<String>, // Use Option to handle absence of a session
// }

// pub async fn signup() -> impl Responder {
//     let template = SignupTemplate {
//         session_id: Some("example_session_id".to_string()), // Example session ID
//     };
 
//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(template.render().unwrap())
// }

// // Template for the signin page
// #[derive(Template)]
// #[template(path = "signin.html")]
// pub struct SigninTemplate {
//     pub session_id: Option<String>, // Use Option to handle absence of a session
// }

// pub async fn signin() -> impl Responder {
//     let template = SigninTemplate {
//         session_id: Some("example_session_id".to_string()), // Example session ID
//     };

//     HttpResponse::Ok()
//         .content_type("text/html")
//         .body(template.render().unwrap())
// }
