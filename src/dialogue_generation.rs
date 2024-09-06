use reqwest::Client;
use serde::Deserialize;
use std::env;

/// Represents a message in the choices array from the API response.
#[derive(Deserialize, Debug)]
pub struct ChoiceMessage {
    pub role: String,
    pub content: String,
}

/// Represents a choice from the API response.
#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: usize,
    pub message: ChoiceMessage,
    pub logprobs: Option<serde_json::Value>, // Log probabilities can be null
    pub finish_reason: String,
}

/// Represents the usage statistics from the API response.
#[derive(Deserialize, Debug)]
pub struct Usage {
    pub queue_time: f64,
    pub prompt_tokens: usize,
    pub prompt_time: f64,
    pub completion_tokens: usize,
    pub completion_time: f64,
    pub total_tokens: usize,
    pub total_time: f64,
}

/// Represents the complete response from the API.
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    pub system_fingerprint: String,
    pub x_groq: serde_json::Value, // Assuming this can vary, so use Value
}

/// Sends a message to the API and returns the JSON response.
///
/// # Arguments
///
/// * `input` - A string slice that holds the user message.
///
/// # Returns
///
/// * `Result<ApiResponse, Box<dyn std::error::Error>>` - A result containing the API response or an error.
pub async fn send_message(input: &str) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY not set");

    // Create request body
    let request_body = serde_json::json!({
        "messages": [
            {
                "role": "user",
                "content": input
            }
        ],
        "model": "llama3-8b-8192"
    });

    // Send request to the API
    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    // Check if the response is successful
    if response.status().is_success() {
        let json_response: ApiResponse = response.json().await?;
        Ok(json_response)
    } else {
        // Handle non-successful responses
        let status = response.status();
        let error_message = response.text().await.unwrap_or_else(|_| "Failed to read error message".to_string());
        Err(format!("Request failed with status: {} - {}", status, error_message).into())
    }
}
