use std::env;

use crate::config::config::load_from_file;

mod config;
mod config_tests;

const SEND_MESSAGE_ENDPOINT: &str = "/rest/send";
const MESSAGE_FORM_PROPERTY: &str = "message";
const HEADER_TOKEN: &str = "token";

const VERSION: &str = "1.0.0";

fn main() {
    println!("Telegram Bot REST Client v{}", VERSION);

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let message = &args[1];

        match load_from_file("telegram-bot-rest-client.conf") {
            Ok(config) => {
                println!("config has been loaded from file");
                send_message(&config.base_url, &config.auth_token, &message);

            }
            Err(_error) => println!("error: unable to load config from file")
        }

    } else { println!("[!] error: message expected") }
}

fn send_message(base_url: &str, auth_token: &str, message: &str) {
    let request_url = format!("{}{}", base_url, SEND_MESSAGE_ENDPOINT);

    let encoded_message = base64::encode(message);

    let params = [(MESSAGE_FORM_PROPERTY, encoded_message)];

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&request_url)
        .header(HEADER_TOKEN, auth_token)
        .form(&params)
        .send().unwrap();

    let status: reqwest::StatusCode = resp.status();

    if status == reqwest::StatusCode::OK {
        println!("message has been sent");

    } else { println!("error, response code was {}", status) }
}
