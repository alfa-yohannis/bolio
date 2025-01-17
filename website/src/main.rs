use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use askama::Template;

// Askama template for rendering
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
    header: &'a str,
}

async fn index() -> Result<HttpResponse> {
    let template = IndexTemplate {
        title: "Bolio",
        header: "Welcome to Bolio",
    };

    let body = template.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running at http://127.0.0.1:50785");

    HttpServer::new(|| {
        App::new()
            // Serve static files from /static
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:50785")?
    .run()
    .await
}
