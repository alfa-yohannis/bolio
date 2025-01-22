// upload_handler.rs
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use futures_util::TryStreamExt;
use regex::Regex;
use serde_json::json;
use std::fs;
use std::io::Write;

pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut conversion_result = String::new();

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition().unwrap();

        // Extract filename
        let filename = content_disposition
            .get_filename()
            .unwrap_or("default_filename")
            .to_string();

        // Handle email validation
        if field.name() == Some("email") {
            let email_data = field.try_next().await?.unwrap_or_default();
            let email = match String::from_utf8(email_data.to_vec()) {
                Ok(email) => email,
                Err(_) => {
                    return Ok(HttpResponse::BadRequest().json(json!({
                        "status": "error",
                        "message": "Invalid email data"
                    })));
                }
            };
            let email_regex =
                Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

            if !email_regex.is_match(&email) {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Invalid email address"
                })));
            }
        }

        // Create a file on the server and perform conversion
        if field.name() == Some("file") {
            let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
            let mut file = fs::File::create(&filepath)?;

            // Write the chunks to the file
            while let Some(chunk) = field.try_next().await? {
                let data = chunk.as_ref();
                file.write_all(data)?;
            }

            // Simulate video-to-text conversion result
            // Replace this with the actual video-to-text conversion logic
            conversion_result = format!("Extracted text from {}.", filename);
        }
    }

    // Return JSON response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "File uploaded successfully.",
        "text": conversion_result,
    })))
}
