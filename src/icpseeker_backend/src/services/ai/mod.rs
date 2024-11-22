mod service;

pub use service::AIService;
pub use crate::ai_service::analyzer::CVAnalyzer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIChoice {
    pub message: OpenAIMessage,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAIResponse {
    pub id: String,
    pub choices: Vec<OpenAIChoice>,
}

pub const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const OPENAI_MODEL: &str = "gpt-3.5-turbo";

pub struct AIConfig {
    pub api_key: String,
    pub max_tokens: u32,
    pub temperature: f32,
}


impl Default for AIConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(), 
            max_tokens: 150,
            temperature: 0.7,
        }
    }
}