/*

    simple chat routine to interact with Googles gemini-pro.
    The routine exptects the GOOGLE_API_KEY to be set in ENV.

    gemini-pro default conifg:
        temperature: 0.7
        maxOutputTokens: 800
        topP: 0.8
        topK: 10

    There are three special commands:
        1. exit: exits the chat
        2. clear: clears the full chat history/memory
        3. undo: clears the last user and agent prompts 

*/
mod gemini;
mod cliutils;

use std::env;

use crate::gemini::{send_request, GeminiContentMessage, GeminiContentPart, GenerationConfig, GeminiContentRequest};
use crate::cliutils::{get_user_input, special_commands};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let google_api_key: String = env::var("GOOGLE_API_KEY").unwrap();
    let uri = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", google_api_key);

    println!("{esc}c", esc = 27 as char);

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
            temperature: 0.7,
            max_output_tokens: 800,
            top_p: 0.8,
            top_k: 10,  
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
