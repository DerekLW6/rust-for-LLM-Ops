use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;
use std::fs;
use base64;

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhisperTranscriptionRequest {
    // Define the structure according to Whisper's API requirements
    audio_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WhisperTranscriptionResponse {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let client = Client::new();

    // Step 1: Load and encode the MP3 file
    let mp3_path = "./JohnFKennedyInauguralAddress.mp3"; // Update with your file name (Updated)
    let audio_data = fs::read(mp3_path)?;
    let encoded_audio = base64::encode(audio_data);

    // Step 2: Send the encoded audio to Whisper for transcription (adjust the URL and request body as needed)
    let whisper_response = client.post("https://api.openai.com/v1/audio/transcriptions") // Placeholder URL (Updated)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&WhisperTranscriptionRequest { audio_data: encoded_audio })
        .send()
        .await?
        .json::<WhisperTranscriptionResponse>()
        .await?;

    let transcribed_text = whisper_response.text; // Convert this

    // Step 3: Use the transcribed text in your chat completion request
    let request_body = ChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are an expert in history who likes to respond with bullets.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: transcribed_text, // Use the transcribed text here
            },
        ],
    };

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_body = response.json::<ChatCompletionResponse>().await?;
    if let Some(choice) = response_body.choices.get(0) {
        println!("Response: {}", choice.message.content);
    } else {
        println!("No response provided by OpenAI.");
    }

    Ok(())
}
