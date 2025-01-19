// upload_handler.rs
use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt;
use std::fs;
use std::io::Write;

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
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
