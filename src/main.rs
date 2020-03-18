use std::env;
use std::path::Path;

use clap::{App, Arg};

use crate::config::config::load_from_file;

mod config;
mod config_tests;

const SEND_MESSAGE_ENDPOINT: &str = "/rest/send";
const MESSAGE_FORM_PROPERTY: &str = "message";
const HEADER_TOKEN: &str = "token";

const WORK_DIR_ARGUMENT: &str = "work-dir";

const VERSION: &str = "1.0.0";

fn main() {
    let matches = App::new("Telegram Bot REST Client")
        .version(VERSION)
        .author("Eugene Lebedev <duke.tougu@gmail.com>")
        .about("Client for Telegram Bot REST Service")

        .arg(
        Arg::with_name("message")
               .help("message") // Displayed when showing help info
               .index(1) // Set the order in which the user must
               .required(true)
        )
        .arg(
        Arg::with_name(WORK_DIR_ARGUMENT)
                .long(WORK_DIR_ARGUMENT)
                .takes_value(true)
                .help("working directory")
        ).get_matches();

    if matches.is_present(WORK_DIR_ARGUMENT) {
        let work_dir = Path::new(matches.value_of(WORK_DIR_ARGUMENT).unwrap());
        env::set_current_dir(&work_dir).expect("unable to set working directory");
        println!("work dir: {}", work_dir.display());
    }

    match matches.value_of("message") {
        Some(message) => {
            match load_from_file("tbrc.conf") {
                Ok(config) => {
                    println!("config has been loaded from file");
                    send_message(&config.base_url, &config.auth_token, &message);

                }
                Err(_error) => println!("error: unable to load config from file")
            }
        }
        None => panic!("message argument required")
    }
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
