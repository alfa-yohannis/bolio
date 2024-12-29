mod audio_transcription;

use audio_transcription::TranscriptionConfig;

fn main() {
    let config = TranscriptionConfig {
        video_path: "videos/video1.webm".to_string(),
        aac_audio_path: "output/output_audio.aac".to_string(),
        wav_audio_path: "output/output_audio.wav".to_string(),
        model_path: "models/ggml-tiny-q5_1.bin".to_string(),
        transcription_path: "transcription/transcription.txt".to_string(),
        language: "en".to_string(),
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

    println!("🎉 Transcription completed successfully!");
}
