// upload_handler.rs
use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures_util::TryStreamExt;
use regex::Regex;
use serde_json::json;
use std::fs;
use std::io::Write;

use crate::audio_transcription::TranscriptionConfig;

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

            // cretae a new filename with a random UUID
            let uuid_filename = format!("{}.{}", uuid::Uuid::new_v4(), "mp4");
            let filepath = format!("./{}/{}", temp_dir, uuid_filename);
            let mut file = fs::File::create(&filepath)?;

            // Write the chunks to the file
            while let Some(chunk) = field.try_next().await? {
                let data = chunk.as_ref();
                file.write_all(data)?;
            }

            let config = TranscriptionConfig {
                video_path: filepath.to_string(),
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

            // Simulate video-to-text conversion result
            // Load the text from the transcription file
            let transcription_text = fs::read_to_string(&config.transcription_path)?;

            // Remove the audio files, transcription file, and uploaded file
            fs::remove_file(&config.aac_audio_path)?;
            fs::remove_file(&config.wav_audio_path)?;
            fs::remove_file(&config.transcription_path)?;
            fs::remove_file(&config.video_path)?;

            conversion_result = transcription_text;

            // // Replace this with the actual video-to-text conversion logic
            // conversion_result = format!("Extracted text from {}.", filename);
        }
    }

    // Return JSON response
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "File uploaded successfully.",
        "text": conversion_result,
    })))
}
