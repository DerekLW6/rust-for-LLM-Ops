use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let client = Client::new();

    let request_body = ChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a poetic assistant, skilled in explaining complex programming concepts with creative flair.".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: "Compose a poem that explains the concept of recursion in programming.".to_string(),
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
