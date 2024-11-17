pub mod models;

use axum::{http::StatusCode, routing::post, Json, Router};
use chat::ChatApi;
use log::info;
use openai_api_rust::chat::ChatBody;
use openai_api_rust::*;
use regex::Regex;
use serde_json::Value;
use tower_http::cors::CorsLayer;

use models::{ChatInput, BASE_URL};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/chat", post(chat))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn chat(Json(chat_input): Json<ChatInput>) -> (StatusCode, Json<Value>) {
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, BASE_URL);

    let country = chat_input.country;
    let city = chat_input.city;
    let query = chat_input.query;

    let messages = gen_message(&country, &city, &query);

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
    let extracted_json = extract_json(&content).unwrap();
    info!("Content: {:?}", extracted_json);

    (StatusCode::OK, Json(extracted_json))
}

pub fn gen_message(country: &str, city: &str, query: &str) -> Vec<Message> {
    let system_role =
        "あなたは優秀な観光ガイドです。主要な観光地はもちろん穴場な観光地まで知っています。";
    let example_input = r#"
次の前提を踏まえて、旅行のタイムラインを作成してください。

クエリ: 札幌の観光地の情報を教えてください
前提: 北海道
"#;
    let example_output = r#"
以下の形式で出力してください:
{
  "time": "10:00",
  "place": "何とかサービスエリア",
  "activity_name": "ソフトクリームを食べる",
  "type": "food"
},
{
  "time": "12:00",
  "place": "どこかのレストラン",
  "activity_name": "ソフトクリームを食べる",
  "type": "food"
}
"#;

    vec![
        Message {
            role: Role::System,
            content: system_role.to_string(),
        },
        Message {
            role: Role::User,
            content: format!(
                "{}\n\n{}",
                example_input, example_output
            ),
        },
        Message {
            role: Role::User,
            content: format!(
                "私は観光客で、{}の{}を訪れています。{}についてこの国での情報を提供してくれますか？",
                country, city, query
            ),
        },
    ]
}

pub fn extract_json(input: &str) -> Option<Value> {
    let re = Regex::new(r#"```json\s*(?P<json>\{.*?\})\s*```"#).unwrap();

    if let Some(captures) = re.captures(input) {
        if let Some(json_str) = captures.name("json") {
            let json: Value = serde_json::from_str(json_str.as_str()).ok()?;
            return Some(json);
        }
    }
    None
}
