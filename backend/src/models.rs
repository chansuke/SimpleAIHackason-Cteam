use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatInput {
    pub country: String,
    pub city: String,
    pub query: String,
}

pub const BASE_URL: &str = "https://api.openai.com/v1/";

pub const FETCH_URL: &str = "https://0a85-152-117-252-49.ngrok-free.app/search?keyword=";
