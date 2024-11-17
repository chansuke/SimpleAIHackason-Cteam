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
    let re = Regex::new(r#"```json\s*(?P<json>[\s\S]*?)\s*```"#).unwrap();

    if let Some(captures) = re.captures(input) {
        if let Some(json_str) = captures.name("json") {
            println!("Extracted JSON: {}", json_str.as_str());
            let json_data = json_str.as_str().trim();
            if json_data.starts_with('{') && json_data.contains("},") {
                let wrapped_json = format!("[{}]", json_data);
                let json: Value = serde_json::from_str(&wrapped_json).ok()?;
                return Some(json);
            } else {
                let json: Value = serde_json::from_str(json_data).ok()?;
                return Some(json[0].clone());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_json() {
        let data = "もちろんです！北海道は自然の美しさと豊かな文化が魅力の観光地です。札幌を中心に、いくつかのおすすめ観光地とアクティビティをタイムライン形式でご紹介します。\n\n```json\n{\n  \"time\": \"09:00\",\n  \"place\": \"大通公園\",\n  \"activity_name\": \"散策と写真撮影\",\n  \"type\": \"sightseeing\"\n},\n{\n  \"time\": \"10:30\",\n  \"place\": \"札幌時計台\",\n  \"activity_name\": \"歴史的建造物の見学\",\n  \"type\": \"sightseeing\"\n},\n{\n  \"time\": \"12:00\",\n  \"place\": \"札幌ラーメン横丁\",\n  \"activity_name\": \"札幌ラーメンを食べる\",\n  \"type\": \"food\"\n}\n```";

        if let Some(json_value) = extract_json(data) {
            assert_eq!(
                json_value[0],
                json!({
                    "time": "09:00",
                    "place": "大通公園",
                    "activity_name": "散策と写真撮影",
                    "type": "sightseeing"
                })
            );
        } else {
            panic!("Failed to extract JSON");
        }
    }
}
