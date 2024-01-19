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
use clap::{Parser, Subcommand};

mod gemini;
mod cliutils;
mod chatroutine;



#[derive(Parser)]
#[command(name = "gemini-cli")]
#[command(author = "Max Mekiska. <maxmekiska@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "CLI to interact with Googles gemini-pro LLM.", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    Chat {
        #[arg(short, long, default_value_t = 0.7, help = "Model temperature.")]
        temperature: f64,

        #[arg(short, long, default_value_t = 800, help = "Maximum tokens the model should generate.")]
        max_output_tokens: i32,

        #[arg(short, long, default_value_t = 0.8, help = "topP.")]
        p_top: f64,

        #[arg(short, long, default_value_t = 10, help = "topK.")]
        k_top: i32,
    },
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let args = Args::parse();

    match &args.command {

        Some(Commands::Chat { temperature, max_output_tokens, p_top, k_top }) => {
            chatroutine::run_chat(*temperature, *max_output_tokens, *p_top, *k_top).await
        }
        None => Ok({})
    }
}
