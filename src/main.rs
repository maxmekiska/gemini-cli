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
mod chatroutine;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    chatroutine::run_chat(0.7, 800, 0.8, 10).await?;

    Ok(())
}
