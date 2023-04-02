use rusoto_core::Region;
use rusoto_polly::{Polly, PollyClient, SynthesizeSpeechInput};
use base64;

pub async fn speak(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Set up a client with your AWS credentials and the desired region
    let client = PollyClient::new(Region::EuWest2);

    // Set up a request to synthesize speech from text
    let input = SynthesizeSpeechInput {
        engine: None,
        language_code: Some("en-US".to_owned()),
        lexicon_names: None,
        output_format: "mp3".to_owned(),
        sample_rate: None,
        speech_mark_types: None,
        text: content.to_owned(),
        text_type: None,
        voice_id: "Joanna".to_owned(),
    };

    // Send the request to AWS Polly and get back an audio stream
    let response = client.synthesize_speech(input).await?;
    let audio_stream = response.audio_stream.unwrap();
    Ok(base64::encode(audio_stream))
}

pub async fn get_polly(ret: String) -> String {
    // Return base64-encoded text-to-speech mp3

    // Polly has max char count of 3000 chars. Chunk body and run text-to-speech on each chunk
    // then collate mp3 data
    let mail_body_chunks = chunker(ret);
    let mut recorded_chunks = Vec::new();
    for chunk in mail_body_chunks {
        let recording = loop {
            match speak(&chunk).await {
                Ok(b64_recording) => break b64_recording,
                Err(e) => {
                    eprintln!("Error Querying AWS Polly: [{e}]. Retrying.");
                    continue;
                }
            }
        };
        recorded_chunks.push(recording);
    }

    let recording: Vec<Vec<u8>> = recorded_chunks.into_iter().map(|chunk| base64::decode(chunk).unwrap()).collect();
    let recording = recording.concat();
    base64::encode(recording)
}

fn chunker(text: String) -> Vec<String> {
    const MAX_CHUNK_SIZE: usize = 3000;
    let char_count = text.chars().count();
    if char_count <= MAX_CHUNK_SIZE {
        return vec![text];
    }

    println!("Chunking. Size: {char_count}");
    let mut chunks = Vec::new();
    let mut start = 0;
    while start < char_count {
        let end = (start + MAX_CHUNK_SIZE).min(char_count);
        let chunk = text[start..end].to_owned();
        chunks.push(chunk);
        start = end;
    }
    chunks
}
