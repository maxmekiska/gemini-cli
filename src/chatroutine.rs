use std::env;

use crate::gemini::{send_request, GeminiContentMessage, GeminiContentPart, GenerationConfig, GeminiContentRequest};
use crate::cliutils::{get_user_input, special_commands};



pub async fn run_chat(temperature: f64, max_output_tokens: i32, top_p: f64 , top_k: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let google_api_key: String = env::var("GOOGLE_API_KEY").unwrap();
    let uri = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", google_api_key);

    let mut conversation_history: Vec<GeminiContentMessage> = Vec::new();

    loop {
        let user_text = get_user_input();

        let action = special_commands(&user_text, &mut conversation_history);

        if action == 1 {
            break;
        } else if action == 2 {
            continue;
        }

        let user_message = GeminiContentMessage {
            role: String::from("user"),
            parts: vec![GeminiContentPart { text: user_text.trim().to_string() }],
        };

        conversation_history.push(user_message.clone());

        let llm_config = GenerationConfig {
            temperature: temperature,
            max_output_tokens: max_output_tokens,
            top_p: top_p,
            top_k: top_k,  
        };

        let gemini_request = GeminiContentRequest {
            contents: conversation_history.clone(),
            generation_config: llm_config,
        };


        if let Ok(response) = send_request(&uri, &gemini_request).await {
            if let Some(candidate) = response.candidates.get(0) {
                println!(">\x1b[32m<\x1b[0m {}", candidate.content.parts[0].text);
                conversation_history.push(candidate.content.clone());
            }
        } else {
            println!(">\x1b[31m<\x1b[0m Error processing request. Please try again.");
        }
    }

    Ok(())
}
