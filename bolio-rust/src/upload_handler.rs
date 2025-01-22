// upload_handler.rs
use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt;
use regex::Regex;
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

        // Handle email validation
        if field.name() == Some("email") {
            let email_data = field.try_next().await?.unwrap_or_default();
            let email = match String::from_utf8(email_data.to_vec()) {
                Ok(email) => email,
                Err(_) => return Ok(HttpResponse::BadRequest().body("Invalid email data")),
            };
            let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();

            if !email_regex.is_match(&email) {
                return Ok(HttpResponse::BadRequest().body("Invalid email address"));
            }
        }

        // Create a file on the server
        if field.name() == Some("file") {
            let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
            let mut file = fs::File::create(filepath)?;

            // Write the chunks to the file
            while let Some(chunk) = field.try_next().await? {
                let data = chunk.as_ref();
                file.write_all(data)?;
            }
        }
    }

    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}


// // upload_handler.rs
// use actix_multipart::Multipart;
// use actix_web::{Error, HttpResponse};
// use futures_util::TryStreamExt;
// use std::fs;
// use std::io::Write;

// pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     while let Some(mut field) = payload.try_next().await? {
//         let content_disposition = field.content_disposition().unwrap();

//         // Extract filename
//         let filename = content_disposition
//             .get_filename()
//             .unwrap_or("default_filename")
//             .to_string();

//         // Create a file on the server
//         let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
//         let mut file = fs::File::create(filepath)?;

//         // Write the chunks to the file
//         while let Some(chunk) = field.try_next().await? {
//             let data = chunk.as_ref();
//             file.write_all(data)?;
//         }
//     }

//     Ok(HttpResponse::Ok().body("File uploaded successfully"))
// }
