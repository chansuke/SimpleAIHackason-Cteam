use axum::{http::StatusCode, routing::post, Json, Router};
use chat::ChatApi;
use log::info;
use openai_api_rust::chat::ChatBody;
use openai_api_rust::*;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/chat", post(chat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn chat(Json(chat_input): Json<ChatInput>) -> (StatusCode, Json<String>) {
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    let country = chat_input.country;
    let city = chat_input.city;
    let query = chat_input.query;

    let messages = vec![Message {
        role: Role::User,
        content: format!(
            "I am a tourist visiting {}, {}. Can you provide information about {} in this country?",
            city, country, query
        ),
    }];

    let chat_body = ChatBody {
        model: "gpt-4o".to_string(),
        messages,
        max_tokens: Some(200),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };
    info!("Chat body: {:?}", chat_body);

    let response = openai.chat_completion_create(&chat_body);
    let choice = response.unwrap().choices;
    let message = choice[0].message.as_ref().unwrap();
    let content = message.content.clone();
    info!("Content: {:?}", content);

    (StatusCode::OK, Json(content))
}

#[derive(Debug, Deserialize)]
struct ChatInput {
    pub country: String,
    pub city: String,
    pub query: String,
}
