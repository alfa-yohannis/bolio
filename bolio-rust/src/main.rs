use hound::WavReader;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

fn main() {
    // Hardcoded paths for model, video, intermediate audio, and transcription output
    let video_path = "videos/video1.webm";
    let aac_audio_path = "output/output_audio.aac";
    let wav_audio_path = "output/output_audio.wav";
    // let model_path = "models/ggml-base.bin";
    let model_path = "models/ggml-tiny-q5_1.bin";
    // let model_path = "models/ggml-large-v3.bin";
    let transcription_path = "transcription/transcription.txt";
    let language = "en";

    // Ensure necessary directories exist
    fs::create_dir_all("output").expect("Failed to create output directory");
    fs::create_dir_all("transcription").expect("Failed to create transcription directory");

    // Step 1: Extract audio from the video as AAC
    println!("🎬 Extracting audio from video...");
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            video_path,
            "-vn",
            "-acodec",
            "aac",
            "-y",
            aac_audio_path,
        ])
        .status()
        .expect("Failed to execute ffmpeg (audio extraction)");

    if status.success() {
        println!("✅ Audio extracted to '{}'", aac_audio_path);
    } else {
        panic!("❌ Failed to extract audio from video.");
    }

    // Step 2: Convert AAC audio to WAV format (16kHz, mono)
    println!("🎵 Converting audio to 16kHz WAV format...");
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            aac_audio_path,
            "-ar", 
            "16000",
            "-ac",
            "1",
            "-c:a",
            "pcm_s16le",
            "-y",
            wav_audio_path,
        ])
        .status()
        .expect("Failed to execute ffmpeg (audio conversion)");

    if status.success() {
        println!("✅ Audio converted to '{}'", wav_audio_path);
    } else {
        panic!("❌ Failed to convert audio to WAV format.");
    }

    // Step 3: Load audio samples from the WAV file
    println!("🎤 Loading audio samples...");
    let samples: Vec<i16> = WavReader::open(wav_audio_path)
        .expect("Failed to open WAV file")
        .into_samples::<i16>()
        .map(|x| x.expect("Failed to read audio sample"))
        .collect();

    // Step 4: Load the Whisper model
    println!("🧠 Loading Whisper model...");
    let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())
        .expect("Failed to load Whisper model");

    let mut state = ctx.create_state().expect("Failed to create Whisper state");

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_language(Some(&language));
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);

    // Step 5: Convert samples to f32 format
    println!("🔄 Converting samples to Vec f32 and convert_integer_to_float_audio");
    let mut inter_samples = vec![Default::default(); samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
        .expect("Failed to convert audio data");

    // let samples = whisper_rs::convert_stereo_to_mono_audio(&inter_samples)
    //     .expect("Failed to convert audio data to mono");

    let samples = inter_samples;

    // Step 6: Run Whisper on the samples
    println!("📝 Running Whisper transcription...");
    state
        .full(params, &samples[..])
        .expect("Failed to run Whisper model");

    // Step 7: Fetch transcription results and save to a file
    let num_segments = state
        .full_n_segments()
        .expect("Failed to get number of segments");

    let mut transcribed_text = String::new();

    for i in 0..num_segments {
        let segment = state
            .full_get_segment_text(i)
            .expect("Failed to get segment text");
        let start_timestamp = state
            .full_get_segment_t0(i)
            .expect("Failed to get segment start timestamp");
        let end_timestamp = state
            .full_get_segment_t1(i)
            .expect("Failed to get segment end timestamp");

        // Print to console with timestamps
        println!(
            "[{} - {}]: {}",
            format_timestamp(start_timestamp.try_into().unwrap()),
            format_timestamp(end_timestamp.try_into().unwrap()),
            segment
        );

        // Append only the text to the final transcription output
        transcribed_text.push_str(&format!("{}\n", segment));
    }

    // Step 8: Save transcription to file
    let mut file = File::create(transcription_path).expect("Failed to create transcription file");
    file.write_all(transcribed_text.as_bytes())
        .expect("Failed to write transcription to file");

    println!("✅ Transcription saved to '{}'", transcription_path);
    println!("🎉 Transcription completed successfully!");
}

/// Helper function to format timestamps as minutes:seconds
fn format_timestamp(t: i32) -> String {
    let seconds = t as f32 / 100.0;
    let minutes = (seconds / 60.0).floor();
    let remaining_seconds = seconds % 60.0;
    format!("{:02}:{:05.2}", minutes as u32, remaining_seconds)
}
