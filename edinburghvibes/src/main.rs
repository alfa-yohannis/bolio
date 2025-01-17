use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Serve the `index.html` file as the root
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
