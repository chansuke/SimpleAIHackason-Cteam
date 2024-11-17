use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatInput {
    pub country: String,
    pub city: String,
    pub query: String,
}

pub const BASE_URL: &str = "https://api.openai.com/v1/";
