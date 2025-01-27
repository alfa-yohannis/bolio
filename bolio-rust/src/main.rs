mod add_credit_handler; // Add add_credit_handler module
mod audio_transcription;
mod index_handler;
mod middlewares;
mod models;
mod schema;
mod signin_handler; // Add signin_handler module
mod signout_handler; // Add signout_handler module
mod signup_handler;
mod video2text_handler;

use actix_files as fs;
use actix_web::{error, web, App, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager}; // Use Diesel's r2d2 integration
use dotenvy::dotenv;
use log::info;
use middlewares::session::SessionMiddleware;
use std::env;
use std::fs as std_fs;

// Type alias for the R2D2 connection pool
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting server...");

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
            .wrap(SessionMiddleware)
            // .default_service(web::route().to(|| HttpResponse::NotFound()))
            .app_data(web::Data::new(pool.clone())) // Pass the database pool to the app
            .service(fs::Files::new("/static", "./static").show_files_listing()) // Serve static files
            .route("/", web::get().to(index_handler::index)) // Root view route
            .route("/upload", web::post().to(video2text_handler::upload_file)) // File upload route // File upload route
            .route("/signup", web::get().to(signup_handler::signup)) // Signup page route
            .route("/signup", web::post().to(signup_handler::handle_signup)) // Signup form submission
            .route("/signin", web::get().to(signin_handler::signin)) // Login page route
            .route("/signin", web::post().to(signin_handler::signin_handler)) // Login form submission
            .route("/signout", web::get().to(signout_handler::signout_handler)) // Sign-out route
            .route("/add_credit", web::get().to(add_credit_handler::show)) // Add credit page route
            .route(
                "/add_credit",
                web::post().to(add_credit_handler::add_credit),
            ) // Add credit page route
            .route("/video2text", web::get().to(video2text_handler::video2text))
        // Sign-out route
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
