use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::io;
use std::io::Write;

const ENDPOINT: &str = "https://api.openai.com/v1/completions";

#[derive(Serialize, Debug)]
struct Payload {
    model: String,
    prompt: String,
    max_tokens: usize,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
    index: usize,
    finish_reason: String,
}

fn get_response(api_key: &str, message: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let payload = Payload {
        model: "text-davinci-003".to_string(),
        prompt: message.to_string(),
        max_tokens: 150,
    };

    println!("Payload {:?}", payload);
    let response: ResponseData = client
        .post(ENDPOINT)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?
        .json()?;
    println!("Response {:?}", response);

    Ok(response.choices[0].text.trim().to_string())
}

fn main() {
    let api_key = if let Ok(value) = env::var("OPENAI_API_KEY") {
        value
    } else {
        println!("Please set OPENAI_API_KEY environment variable");
        return;
    };
    println!("API Key: {}", api_key);

    loop {
        let mut input = String::new();
        print!("You: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();
        if trimmed == "quit" || trimmed == "exit" {
            break;
        }

        match get_response(&api_key, trimmed) {
            Ok(response) => println!("ChatGPT: {}", response),
            Err(err) => println!("Error: {}", err),
        }
    }
}
