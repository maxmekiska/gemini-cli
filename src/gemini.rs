use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiContentPart {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeminiContentMessage {
    pub role: String,
    pub parts: Vec<GeminiContentPart>,
}

#[derive(Serialize, Debug)]
pub struct GeminiContentRequest {
    pub contents: Vec<GeminiContentMessage>,
    #[serde(rename = "generationConfig")]
    pub generation_config: GenerationConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeminiContentResponse {
    pub candidates: Vec<GeminiContentCandidate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeminiContentCandidate {
    pub content: GeminiContentMessage,
}

#[derive(Serialize, Debug)]
pub struct GenerationConfig {
    pub temperature: f64,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: i32,
    #[serde(rename = "topP")]
    pub top_p: f64,
    #[serde(rename = "topK")]
    pub top_k: i32,
}


pub async fn send_request(uri: &str, gemini_request: &GeminiContentRequest) -> Result<GeminiContentResponse, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);

    let body = Body::from(serde_json::to_vec(&gemini_request)?);

    let req = Request::post(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body)
        .unwrap();

    let res = client.request(req).await?;

    if !res.status().is_success() {
        let error_message = format!("Google Request failed with status: {}", res.status());
        
        let error_body = hyper::body::aggregate(res).await?;
        let plain_json_response = String::from_utf8_lossy(error_body.chunk()).to_string();
        eprintln!("Plain JSON Response: {}", plain_json_response);

        return Err(error_message.into());
    }

    let body = hyper::body::aggregate(res).await?;
    let json: GeminiContentResponse = serde_json::from_reader(body.reader())?;
    Ok(json)
}
