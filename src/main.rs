use base64;
use clap::{Arg, Command};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let help = Arg::new("agent")
        .short('A')
        .long("agent")
        .help("User-Agent name");

    Ok(())
}
