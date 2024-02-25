use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let client = Client::new();
    
    // Path to your audio file
    let file_path = "./JohnFKennedyInauguralAddress.mp3"; // Update this with your file path
    
    // Open the audio file
    let mut file = File::open(file_path).await?;
    
    // Read the audio data into a buffer
    let mut audio_data = Vec::new();
    file.read_to_end(&mut audio_data).await?;
    
    // Step 1: Transcribe the audio file
    let form_data = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(audio_data).file_name("audio.mp3"))
        .text("model", "whisper-1"); // Specify the model you want to use for transcription
    
    // Send the transcription request
    let response = client.post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form_data)
        .send()
        .await?;
    
    if response.status().is_success() {
        let transcribed_text = response.text().await?;
        println!("Transcription Worked!!!!");

        // Step 2: Use the transcribed text in your chat completion request
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

        // Send the summarization request
        let response = client.post("https://api.openai.com/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await?;

        // Process and print the summarization response
        let response_body = response.json::<ChatCompletionResponse>().await?;
        if let Some(choice) = response_body.choices.get(0) {
            println!("Response: {}", choice.message.content);
        } else {
            println!("No response provided by OpenAI.");
        }
    } else {
        eprintln!("Error from Whisper: {}", response.text().await?);
    }

    Ok(())
}
