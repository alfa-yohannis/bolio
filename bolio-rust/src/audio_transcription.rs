// audio_transcription.rs

use hound::WavReader;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

/// Configuration struct to hold paths and parameters
pub struct TranscriptionConfig {
    pub video_path: String,
    pub aac_audio_path: String,
    pub wav_audio_path: String,
    pub model_path: String,
    pub transcription_path: String,
    pub language: String,
}

impl TranscriptionConfig {
    /// Ensure required directories exist
    pub fn ensure_directories(&self) {
        fs::create_dir_all("output").expect("Failed to create output directory");
        fs::create_dir_all("transcription").expect("Failed to create transcription directory");
    }

    /// Extracts audio from a video and saves it as an AAC file.
    pub fn extract_audio(&self) {
        println!("🎬 Extracting audio from video...");
        let status = Command::new("/data2/ffmpeg/bin/ffmpeg")
            .args([
                "-i",
                &self.video_path,
                "-vn",
                "-acodec",
                "aac",
                "-y",
                &self.aac_audio_path,
            ])
            .status()
            .expect("Failed to execute ffmpeg (audio extraction)");

        if status.success() {
            println!("✅ Audio extracted to '{}'", self.aac_audio_path);
        } else {
            panic!("❌ Failed to extract audio from video.");
        }
    }

    /// Converts AAC audio to WAV format (16kHz, mono).
    pub fn convert_audio(&self) {
        println!("🎵 Converting audio to 16kHz WAV format...");
        let status = Command::new("/data2/ffmpeg/bin/ffmpeg")
            .args([
                "-i",
                &self.aac_audio_path,
                "-ar",
                "16000",
                "-ac",
                "1",
                "-c:a",
                "pcm_s16le",
                "-y",
                &self.wav_audio_path,
            ])
            .status()
            .expect("Failed to execute ffmpeg (audio conversion)");

        if status.success() {
            println!("✅ Audio converted to '{}'", self.wav_audio_path);
        } else {
            panic!("❌ Failed to convert audio to WAV format.");
        }
    }

    /// Loads audio samples from a WAV file.
    pub fn load_audio_samples(&self) -> Vec<i16> {
        println!("🎤 Loading audio samples...");
        WavReader::open(&self.wav_audio_path)
            .expect("Failed to open WAV file")
            .into_samples::<i16>()
            .map(|x| x.expect("Failed to read audio sample"))
            .collect()
    }

    /// Runs Whisper transcription and saves results to a file.
    pub fn transcribe_audio(&self, samples: Vec<i16>) {
        println!("🧠 Loading Whisper model...");
        let ctx =
            WhisperContext::new_with_params(&self.model_path, WhisperContextParameters::default())
                .expect("Failed to load Whisper model");

        let mut state = ctx.create_state().expect("Failed to create Whisper state");

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some(&self.language));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        println!("🔄 Converting samples to Vec f32...");
        let mut inter_samples = vec![Default::default(); samples.len()];
        whisper_rs::convert_integer_to_float_audio(&samples, &mut inter_samples)
            .expect("Failed to convert audio data");

        println!("📝 Running Whisper transcription...");
        state
            .full(params, &inter_samples[..])
            .expect("Failed to run Whisper model");

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

            println!(
                "[{} - {}]: {}",
                format_timestamp(start_timestamp.try_into().unwrap()),
                format_timestamp(end_timestamp.try_into().unwrap()),
                segment
            );
            transcribed_text.push_str(&format!("{}", segment.trim()));
        }

        let mut file: File =
            File::create(&self.transcription_path).expect("Failed to create transcription file");
        file.write_all(transcribed_text.as_bytes())
            .expect("Failed to write transcription to file");

        println!("✅ Transcription saved to '{}'", self.transcription_path);
    }
}

/// Formats timestamps as minutes:seconds.
fn format_timestamp(t: i32) -> String {
    let seconds = t as f32 / 100.0;
    let minutes = (seconds / 60.0).floor();
    let remaining_seconds = seconds % 60.0;
    format!("{:02}:{:05.2}", minutes as u32, remaining_seconds)
}
