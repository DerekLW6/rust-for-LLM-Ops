use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::env;

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
    
    // Construct the request with the file attachment
    let form_data = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(audio_data).file_name("audio.mp3"))
        .text("model", "whisper-1"); // Specify the model you want to use for transcription
    
    // Send the request
    let response = client.post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form_data)
        .send()
        .await?;
    
    if response.status().is_success() {
        let response_body = response.text().await?;
        println!("Transcription Response: {}", response_body);
    } else {
        eprintln!("Error: {}", response.text().await?);
    }

    Ok(())
}
