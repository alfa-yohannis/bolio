use std::path::Path;
use A2VConverter::AudioVideoConverter;

fn main() {
    // Define file paths
    let video_file = "videos/video1.webm";
    let output_file = "audio1.mp3";

    println!("AA");

    // Check if the video file exists
    if !Path::new(video_file).exists() {
        println!("Error: Video file '{}' does not exist.", video_file);
        std::process::exit(1);
    } else {
        println!("File does exists");
    }
    println!("CCC");

    // Proceed with conversion
    match AudioVideoConverter::convert_video_to_audio(video_file, output_file) {
        Ok(_) => println!("Video converted to audio successfully!"),
        Err(e) => println!("Error during conversion: {:?}", e),
    }
}
