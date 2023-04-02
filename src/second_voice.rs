use tts::{Tts, Voice};

pub fn save_tts_to_file() {
    // Create a new Tts instance
    let tts = Tts::default()?;

    // Get a list of available voices
    let voices = tts.list_voices()?;
    println!("Available voices:");
    for voice in &voices {
        println!(" - {} ({})", voice.name, voice.language);
    }

    // Choose a voice and synthesize speech
    let voice = voices.iter().find(|v| v.name == "Alex").unwrap();
    let audio_data = tts.synthesize("Hello world!", Some(voice))?;

    // Save the audio data to an MP3 file
    std::fs::write("output.mp3", audio_data)?;
}