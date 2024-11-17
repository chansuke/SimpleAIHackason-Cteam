use log::info;
use openai_api_rust::chat::*;
use openai_api_rust::*;

fn main() {
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-4o".to_string(),
        max_tokens: Some(7),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        // TODO: pass role and content as parameters
        messages: vec![Message {
            role: Role::User,
            content: "Hello!".to_string(),
        }],
    };
    let response = openai.chat_completion_create(&body);
    info!("response: {:?}", response);
    let choice = response.unwrap().choices;
    info!("{:?}", choice);

    let message = &choice[0].message.as_ref().unwrap();
    println!("{:?}", message);
}
