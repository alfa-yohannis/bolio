mod models;
mod schema;
mod signup_handler;
mod template_handler;
mod upload_handler;

use actix_files as fs;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager}; // Use Diesel's r2d2 integration
use dotenvy::dotenv;
use std::env; 
use std::fs as std_fs; 

// Type alias for the R2D2 connection pool
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection manager for PostgreSQL
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Ensure the upload directory exists
    std_fs::create_dir_all("./uploads").unwrap();

    // Start the HTTP server 
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Pass the database pool to the app
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/", web::get().to(template_handler::index))                  // Root view route
            .route("/upload", web::post().to(upload_handler::upload_file))       // File upload route
            .route("/signup", web::get().to(template_handler::signup))           // Signup page route
            .route("/signup", web::post().to(signup_handler::handle_signup))     // Signup form submission
            .route("/signin", web::get().to(template_handler::signin))           // Login route
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
