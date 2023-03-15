use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String
}

#[derive(Deserialize)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>
}

#[derive(Debug, Serialize)]
struct OAIRequest {
    prompt: String,
    max_tokens: u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    let oai_token: String = env::var("OAIAPIKEY").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);

    // clears the screen
    println!("{esc}c", esc = 27 as char);

    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut user_txt = String::new();
        stdin()
            .read_line(&mut user_txt)
            .expect("failed to read line");

        println!();
        let sp = Spinner::new(&Spinners::Dots9, "\t\tThinking hard ...".to_string());

        let oai_request = OAIRequest {
            prompt: user_txt,
            max_tokens: 100
        };

        let body = Body::from(serde_json::to_vec(&oai_request)?);

        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(body)
            .unwrap();

        let res = client.request(req).await?;

        let body = hyper::body::aggregate(res).await?;

        let json: OAIResponse = serde_json::from_reader(body.reader())?;

        sp.stop();

        println!();

        println!("{}", json.choices[0].text);

    }

    Ok(())
}