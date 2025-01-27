use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures_util::TryStreamExt;
use regex::Regex;
use serde_json::json;
use std::fs;
use std::io::Write;

use crate::audio_transcription::TranscriptionConfig;
use crate::models::NewConversionTransaction; // Model for inserting a conversion transaction
use crate::models::User;
use crate::schema::{conversion_transactions, users}; // Diesel schema for users and conversion_transactions
use crate::DbPool; // Database connection pool // Assuming you have a User model defined

pub async fn upload_file(
    req: HttpRequest,
    mut payload: Multipart,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let mut conversion_result = String::new();
    let mut source_size = 0; // To track the size of the uploaded file
    let mut target_size = 0; // To simulate the size of the resulting transcription

    let conversion_type = "video-to-text";
    let target_type = "txt";

    let session_id = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());

    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    // Insert conversion transaction and update user credit
    let mut conn = pool.get().expect("Failed to get database connection");

    let user = if let Some(ref username) = username {
        // Retrieve the user by username and session_id
        users::table
            .filter(users::username.eq(&username))
            .filter(users::session_id.eq(session_id.as_deref().unwrap_or("")))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    } else {
        None
    };

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition().unwrap();

        // Extract filename
        let filename = content_disposition
            .get_filename()
            .unwrap_or("default_filename")
            .to_string();

        // Get the file extension
        let source_type = match filename.split('.').last() {
            Some(ext) => ext.to_string(),
            None => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Failed to get file extension"
                })));
            }
        };

        // Validate the file extension
        let valid_extensions = [
            "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "mpg", "mpeg", "m4v", "3gp", "3g2",
            "vob", "ogv", "rm", "rmvb", "asf", "ts", "m2ts", "f4v", "divx", "xvid", "mxf", "hevc",
            "h264", "dv", "drc", "ogm", "ivf", "amv", "av1", "vp9", "qt", "prores",
        ];

        if !valid_extensions.contains(&source_type.as_str()) {
            return Ok(HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Invalid file type"
            })));
        }

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
            // Load configuration from a file
            let config_data = fs::read_to_string("config.json")?;
            let config_json: serde_json::Value = serde_json::from_str(&config_data)?;

            let temp_dir = config_json["temp_dir"].as_str().unwrap_or("temp");
            let model_path = config_json["model_path"]
                .as_str()
                .unwrap_or("models/ggml-tiny-q5_1.bin");
            let language = config_json["language"].as_str().unwrap_or("en");

            // Sanitize the filename
            let _original_filename = sanitize_filename::sanitize(&filename);

            // Create a new filename with a random UUID
            let uuid_filename = format!("{}.{}", uuid::Uuid::new_v4(), "mp4");
            let filepath = format!("./{}/{}", temp_dir, uuid_filename);
            let mut file = fs::File::create(&filepath)?;

            // Write the chunks to the file and calculate the source size
            while let Some(chunk) = field.try_next().await? {
                let data = chunk.as_ref();
                file.write_all(data)?;
                source_size += data.len() as i64; // Accumulate the source file size in bytes

                // Check if username and session_id exist
                if username.is_none() || session_id.is_none() {
                    if source_size > 10 * 1024 * 1024 {
                        return Ok(HttpResponse::BadRequest().json(json!({
                            "status": "error",
                            "message": "File size must be smaller than 10 MB for unauthenticated users."
                        })));
                    }
                }
            }

            let config = TranscriptionConfig {
                video_path: filepath.clone(),
                aac_audio_path: format!("{}/{}.aac", temp_dir, uuid_filename),
                wav_audio_path: format!("{}/{}.wav", temp_dir, uuid_filename),
                model_path: model_path.to_string(),
                transcription_path: format!("{}/{}.txt", temp_dir, uuid_filename),
                language: language.to_string(),
            };

            // Ensure necessary directories exist
            config.ensure_directories();

            // Step 1: Extract audio from the video
            config.extract_audio();

            // Step 2: Convert audio to WAV
            config.convert_audio();

            // Step 3: Load audio samples
            let samples = config.load_audio_samples();

            // Step 4: Transcribe audio
            config.transcribe_audio(samples);

            conversion_result = fs::read_to_string(&config.transcription_path)?;

            // Calculate the target file size based on the conversion result
            target_size = conversion_result.len() as i64;

            // Calculate total credit used (source + target sizes)
            let credit_used = source_size + target_size;

            if let Some(ref username) = username {
                // Retrieve the user by username and session_id
                let user = users::table
                    .filter(users::username.eq(&username))
                    .filter(users::session_id.eq(session_id.as_deref().unwrap_or("")))
                    .first::<User>(&mut conn)
                    .optional()
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

                if let Some(user) = user {
                    // Insert conversion transaction
                    let new_transaction = NewConversionTransaction {
                        user_id: user.id,
                        source_size,
                        target_size,
                        conversion_type: conversion_type.to_string(),
                        source_type: source_type,
                        target_type: target_type.to_string(),
                    };

                    conn.transaction::<_, diesel::result::Error, _>(|conn| {
                        diesel::insert_into(conversion_transactions::table)
                            .values(&new_transaction)
                            .execute(conn)?;

                        // Update user credit
                        diesel::update(users::table.filter(users::id.eq(user.id)))
                            .set(users::credit.eq(users::credit - credit_used))
                            .execute(conn)?;

                        Ok(())
                    }).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

                    // Clean up temporary files
                    fs::remove_file(&filepath)?;
                    fs::remove_file(&config.aac_audio_path)?;
                    fs::remove_file(&config.wav_audio_path)?;
                    fs::remove_file(&config.transcription_path)?;
                } else {
                    // Clean up temporary files
                    fs::remove_file(&filepath)?;
                    fs::remove_file(&config.aac_audio_path)?;
                    fs::remove_file(&config.wav_audio_path)?;
                    fs::remove_file(&config.transcription_path)?;

                    return Ok(HttpResponse::BadRequest().json(json!({
                        "status": "error",
                        "message": "User not found"
                    })));
                }
            } else {
                // Clean up temporary files
                fs::remove_file(&filepath)?;
                fs::remove_file(&config.aac_audio_path)?;
                fs::remove_file(&config.wav_audio_path)?;
                fs::remove_file(&config.transcription_path)?;

                return Ok(HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Username not provided"
                })));
            }

            // conversion_result;
        }
    }

    // Return JSON response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "File uploaded successfully.",
        "text": conversion_result,
    })))
}

// // upload_handler.rs
// use actix_multipart::Multipart;
// use actix_web::{Error, HttpResponse};
// use futures_util::TryStreamExt;
// use regex::Regex;
// use serde_json::json;
// use std::fs;
// use std::io::Write;

// use crate::audio_transcription::TranscriptionConfig;

// pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     let mut conversion_result = String::new();

//     while let Some(mut field) = payload.try_next().await? {
//         let content_disposition = field.content_disposition().unwrap();

//         // Extract filename
//         let filename = content_disposition
//             .get_filename()
//             .unwrap_or("default_filename")
//             .to_string();

//         // Handle email validation
//         if field.name() == Some("email") {
//             let email_data = field.try_next().await?.unwrap_or_default();
//             let email = match String::from_utf8(email_data.to_vec()) {
//                 Ok(email) => email,
//                 Err(_) => {
//                     return Ok(HttpResponse::BadRequest().json(json!({
//                         "status": "error",
//                         "message": "Invalid email data"
//                     })));
//                 }
//             };
//             let email_regex =
//                 Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

//             if !email_regex.is_match(&email) {
//                 return Ok(HttpResponse::BadRequest().json(json!({
//                     "status": "error",
//                     "message": "Invalid email address"
//                 })));
//             }
//         }

//         // Create a file on the server and perform conversion
//         if field.name() == Some("file") {
//             // Load configuration from a file
//             let config_data = fs::read_to_string("config.json")?;
//             let config_json: serde_json::Value = serde_json::from_str(&config_data)?;

//             let temp_dir = config_json["temp_dir"].as_str().unwrap_or("temp");
//             let model_path = config_json["model_path"]
//                 .as_str()
//                 .unwrap_or("models/ggml-tiny-q5_1.bin");
//             let language = config_json["language"].as_str().unwrap_or("en");

//             // Sanitize the filename
//             let _original_filename = sanitize_filename::sanitize(&filename);

//             // cretae a new filename with a random UUID
//             let uuid_filename = format!("{}.{}", uuid::Uuid::new_v4(), "mp4");
//             let filepath = format!("./{}/{}", temp_dir, uuid_filename);
//             let mut file = fs::File::create(&filepath)?;

//             // Write the chunks to the file
//             while let Some(chunk) = field.try_next().await? {
//                 let data = chunk.as_ref();
//                 file.write_all(data)?;
//             }

//             let config = TranscriptionConfig {
//                 video_path: filepath.to_string(),
//                 aac_audio_path: format!("{}/{}.aac", temp_dir, uuid_filename),
//                 wav_audio_path: format!("{}/{}.wav", temp_dir, uuid_filename),
//                 model_path: model_path.to_string(),
//                 transcription_path: format!("{}/{}.txt", temp_dir, uuid_filename),
//                 language: language.to_string(),
//             };

//             // Ensure necessary directories exist
//             config.ensure_directories();

//             // Step 1: Extract audio from the video
//             config.extract_audio();

//             // Step 2: Convert audio to WAV
//             config.convert_audio();

//             // Step 3: Load audio samples
//             let samples = config.load_audio_samples();

//             // Step 4: Transcribe audio
//             config.transcribe_audio(samples);

//             // Simulate video-to-text conversion result
//             // Load the text from the transcription file
//             let transcription_text = fs::read_to_string(&config.transcription_path)?;

//             // Remove the audio files, transcription file, and uploaded file
//             fs::remove_file(&config.aac_audio_path)?;
//             fs::remove_file(&config.wav_audio_path)?;
//             fs::remove_file(&config.transcription_path)?;
//             fs::remove_file(&config.video_path)?;

//             conversion_result = transcription_text;

//             // // Replace this with the actual video-to-text conversion logic
//             // conversion_result = format!("Extracted text from {}.", filename);
//         }
//     }

//     // Return JSON response
//     Ok(HttpResponse::Ok().json(json!({
//         "status": "success",
//         "message": "File uploaded successfully.",
//         "text": conversion_result,
//     })))
// }
