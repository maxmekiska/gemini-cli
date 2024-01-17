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
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::io::{stdin, stdout, Write};


#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeminiContentPart {
    text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GeminiContentMessage {
    role: String,
    parts: Vec<GeminiContentPart>,
}

#[derive(Serialize, Debug)]
struct GeminiContentRequest {
    contents: Vec<GeminiContentMessage>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiContentResponse {
    candidates: Vec<GeminiContentCandidate>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiContentCandidate {
    content: GeminiContentMessage,
}

#[derive(Serialize, Debug)]
struct GenerationConfig {
    temperature: f64,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: i32,
    #[serde(rename = "topP")]
    top_p: f64,
    #[serde(rename = "topK")]
    top_k: i32,
}


fn special_commands(user_text: &str, conversation_history: &mut Vec<GeminiContentMessage>) -> u8 {
    match user_text.trim().to_lowercase().as_str() {
        "exit" => {
            println!("Exiting the program.");
            return 1;
        }
        "clear" => {
            println!("Chat history cleared.");
            conversation_history.clear();
            return 2;
        }
        "undo" if conversation_history.len() >= 2 => {
            conversation_history.pop();
            conversation_history.pop();
            println!("Undone last user input and assistant response.");
            return 2;
        }
        _ => {
            return 0;
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let google_api_key: String = env::var("GOOGLE_API_KEY").unwrap();
    let uri = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}", google_api_key);

    println!("{esc}c", esc = 27 as char);

    let mut conversation_history: Vec<GeminiContentMessage> = Vec::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut user_text = String::new();
        stdin()
            .read_line(&mut user_text)
            .expect("Failed to read line");

        println!("");

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

        let body = Body::from(serde_json::to_vec(&gemini_request)?);

        let req = Request::post(&uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap();

        let res = match client.request(req).await {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Error sending HTTP request: {}", e);
                return Err(e.into());
            }
        };

        let body = hyper::body::aggregate(res).await?;

        let json: GeminiContentResponse = serde_json::from_reader(body.reader())?;
        
        if let Some(candidate) = json.candidates.get(0) {
            println!("gemini-pro: {}", candidate.content.parts[0].text);
            conversation_history.push(candidate.content.clone());
        }

    }

    Ok(())
}
