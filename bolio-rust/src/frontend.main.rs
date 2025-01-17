use actix_multipart::Multipart;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use askama::Template;
use futures_util::TryStreamExt;
use std::fs;
use std::io::Write;

// Define a template for the view
#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

// Handler for rendering the view
async fn index() -> impl Responder {
    let template = IndexTemplate {
        name: "Alfa".to_string(),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// Handler for file upload
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition().unwrap();

        // Extract filename
        let filename = content_disposition
            .get_filename()
            .unwrap_or("default_filename")
            .to_string();

        // Create a file on the server
        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
        let mut file = fs::File::create(filepath)?;

        // Write the chunks to the file
        while let Some(chunk) = field.try_next().await? {
            let data = chunk.as_ref();
            file.write_all(data)?;
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

// Main function to start the Actix server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:50785");

    // Ensure the upload directory exists
    fs::create_dir_all("./uploads").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // Root view route
            .route("/upload", web::post().to(upload_file)) // File upload route
    })
    .bind(("127.0.0.1", 50785))?
    .run()
    .await
    //
}
