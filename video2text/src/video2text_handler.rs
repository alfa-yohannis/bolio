use actix::Addr;
use actix_multipart::Multipart;
use actix_web::Responder;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use askama::Template;
use diesel::prelude::*;
use futures_util::TryStreamExt;
use log::info;
use regex::Regex;
use serde_json::json;
use std::fs;
use std::io::Write;

use crate::audio_transcription::TranscriptionConfig;
use crate::models::NewConversionTransaction; // Model for inserting a conversion transaction
use crate::models::User;
use crate::progress_updater::{ProgressUpdate, ProgressUpdater};
use crate::schema::{conversion_transactions, users}; // Diesel schema for users and conversion_transactions
use crate::DbPool; // Database connection pool // Assuming you have a User model defined

// Template for the video-to-text page
#[derive(Template)]
#[template(path = "video2text.html")]
pub struct Video2TextTemplate {
    pub session_id: Option<String>, // Use Option to handle absence of a session
    pub username: String,           // Use Option to handle absence of a username
}

pub async fn video2text(req: HttpRequest) -> impl Responder {
    // Retrieve session_id and username from cookies
    let session_id = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());
    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string());

    let template = Video2TextTemplate {
        session_id,
        username: username.unwrap_or_else(|| "Guest".to_string()),
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

// pub async fn upload_file(
//     req: HttpRequest,
//     mut payload: Multipart,
//     pool: web::Data<DbPool>,
//     progress_addr: web::Data<Addr<ProgressUpdater>>,
// ) -> Result<HttpResponse, Error> {
pub async fn upload_file(
    req: HttpRequest,
    mut payload: Multipart,
    pool: web::Data<DbPool>,
    progress_addr: web::Data<Addr<ProgressUpdater>>, // WebSocket for progress updates
) -> Result<HttpResponse, Error> {
    info!("upload_file");

    let mut conversion_result = String::new();
    let mut source_size = 0;
    let mut target_size = 0;

    let conversion_type = "video-to-text";
    let target_type = "txt";

    let session_id = req
        .cookie("session_id")
        .map(|cookie| cookie.value().to_string());

    let username = req
        .cookie("username")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Guest".to_string());

    let is_guest = username == "Guest";

    let mut conn = pool.get().expect("Failed to get database connection");

    let user = if !is_guest {
        users::table
            .filter(users::username.eq(&username))
            .filter(users::session_id.eq(session_id.as_deref().unwrap_or("")))
            .first::<User>(&mut conn)
            .optional()
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
    } else {
        None
    };

    // Notify WebSocket that file upload has started
    progress_addr.get_ref().do_send(ProgressUpdate {
        session_id: session_id.clone().unwrap_or_default(),
        message: "Uploading File...".to_string(),
    });

    while let Some(mut field) = payload.try_next().await? {
        let content_disposition = field.content_disposition().unwrap();
        let filename = content_disposition
            .get_filename()
            .unwrap_or("default_filename")
            .to_string();

        if field.name() == Some("email") {
            let email_data = field.try_next().await?.unwrap_or_default();
            let email = String::from_utf8(email_data.to_vec()).unwrap_or_default();
            let email_regex =
                Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

            if !email_regex.is_match(&email) {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Invalid email address"
                })));
            }
        }

        if field.name() == Some("file") {
            let source_type = match filename.split('.').last() {
                Some(ext) => ext.to_string(),
                None => {
                    return Ok(HttpResponse::BadRequest().json(json!({
                        "status": "error",
                        "message": "Failed to get file extension"
                    })));
                }
            };

            let valid_extensions = [
                "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "mpg", "mpeg", "m4v", "3gp",
                "3g2", "vob", "ogv", "rm", "rmvb", "asf", "ts", "m2ts", "f4v", "divx", "xvid",
                "mxf", "hevc", "h264", "dv", "drc", "ogm", "ivf", "amv", "av1", "vp9", "qt",
                "prores",
            ];

            if !valid_extensions.contains(&source_type.as_str()) {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "Invalid file type"
                })));
            }

            let config_data = fs::read_to_string("config.json")?;
            let config_json: serde_json::Value = serde_json::from_str(&config_data)?;

            let temp_dir = config_json["temp_dir"].as_str().unwrap_or("temp");
            let model_path = config_json["model_path"]
                .as_str()
                .unwrap_or("models/ggml-tiny-q5_1.bin");
            let language = config_json["language"].as_str().unwrap_or("en");

            let uuid_filename = format!("{}.{}", uuid::Uuid::new_v4(), source_type);
            let filepath = format!("./{}/{}", temp_dir, uuid_filename);
            let mut file = fs::File::create(&filepath)?;

            while let Some(chunk) = field.try_next().await? {
                let data = chunk.as_ref();
                file.write_all(data)?;
                source_size += data.len() as i64;
            }

            if is_guest && source_size > 10 * 1024 * 1024 {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "status": "error",
                    "message": "File size must be smaller than 10 MB for guests."
                })));
            }

            if let Some(ref user) = user {
                if source_size > user.credit {
                    fs::remove_file(&filepath)?;
                    return Ok(HttpResponse::BadRequest().json(json!({
                        "status": "error",
                        "message": "Insufficient credit for the file size."
                    })));
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

            progress_addr.get_ref().do_send(ProgressUpdate {
                session_id: session_id.clone().unwrap_or_default(),
                message: "Extracting Audio from Video...".to_string(),
            });

            config.extract_audio();

            progress_addr.get_ref().do_send(ProgressUpdate {
                session_id: session_id.clone().unwrap_or_default(),
                message: "Converting Audio to WAV...".to_string(),
            });

            config.convert_audio();

            progress_addr.get_ref().do_send(ProgressUpdate {
                session_id: session_id.clone().unwrap_or_default(),
                message: "Transcribing Audio...".to_string(),
            });

            let samples = config.load_audio_samples();
            config.transcribe_audio(samples);

            conversion_result = fs::read_to_string(&config.transcription_path)?;
            target_size = conversion_result.len() as i64;
            let credit_used = source_size + target_size;

            if let Some(ref user) = user {
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

                    diesel::update(users::table.filter(users::id.eq(user.id)))
                        .set(users::credit.eq(users::credit - credit_used))
                        .execute(conn)?;

                    Ok(())
                })
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            }

            progress_addr.get_ref().do_send(ProgressUpdate {
                session_id: session_id.clone().unwrap_or_default(),
                message: "Processing Completed!".to_string(),
            });

            fs::remove_file(&filepath)?;
            fs::remove_file(&config.aac_audio_path)?;
            fs::remove_file(&config.wav_audio_path)?;
            fs::remove_file(&config.transcription_path)?;
        }
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "File uploaded successfully.",
        "text": conversion_result,
    })))
}
