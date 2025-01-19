mod upload_handler;
mod template_handler;
mod signup_handler;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::fs as std_fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Ensure the upload directory exists
    std_fs::create_dir_all("./uploads").unwrap();

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/", web::get().to(template_handler::index))                  // Root view route
            .route("/upload", web::post().to(upload_handler::upload_file))       // File upload route
            .route("/signup", web::get().to(template_handler::signup)) // Signup page route
            .route("/signup", web::post().to(signup_handler::handle_signup))     // Signup form submission
            .route("/signin", web::get().to(template_handler::signin))           // Login route
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
