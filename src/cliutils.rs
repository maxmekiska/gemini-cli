use std::io::{stdin, stdout, Write};
use std::io::BufRead;

use crate::gemini::GeminiContentMessage;



pub fn get_user_input() -> String {
    print!("> ");
    stdout().flush().unwrap();
    let mut lines = stdin().lock().lines();
    let mut user_text = String::new();
    //stdin().read_line(&mut user_text).expect("Failed to read line");
    while let Some(line) = lines.next() {
        match line {
            Ok(line) => {
                user_text.push_str(&line);
                user_text.push('\n');
                }
            Err(_) => {
        //        eprintln!("Error: {}", err);
                }
            }
        }

    return user_text
}


pub fn special_commands(user_text: &str, conversation_history: &mut Vec<GeminiContentMessage>) -> u8 {
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
