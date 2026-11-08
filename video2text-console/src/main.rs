mod audio_transcription;

use audio_transcription::TranscriptionConfig;
// use std::env;

fn main() {

     // env_logger::init();
     env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Define file paths and model settings
    let video_path =
        // "input/containers.mp4"
        //     .to_string();
        "/data2/OBS/2025-11-14 14-09-45.mkv".to_string();

    let aac_audio_path = "output/audio.aac".to_string();
    let wav_audio_path = "output/audio.wav".to_string();
    let model_path = "models/ggml-tiny-q5_1.bin".to_string();
    let transcription_path = "output/transcription.txt".to_string();
    let language = "id".to_string();

    // Initialize configuration
    let config = TranscriptionConfig {
        video_path,
        aac_audio_path,
        wav_audio_path,
        model_path,
        transcription_path,
        language,
    };

    // Extract audio from video
    config.extract_audio();

    // Convert audio to WAV format
    config.convert_audio();

    // Load audio samples
    let samples = config.load_audio_samples();

    // Perform transcription
    config.transcribe_audio(samples);

    println!("✅ Transcription process completed!");
}
