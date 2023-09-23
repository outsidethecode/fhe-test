use serde::Deserialize;
use reqwest::{Error, header::ACCEPT};

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

async fn post_device() -> Result<(), Error> {
    let url = format!("http://localhost:8000/api/device");
    // the rest is the same as before!
    let json_data = r#"{"fhe_public_key": "pk11111111", "fmc_code": "7890", "mobile_hash": "h12345"}"#;

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .body(json_data.to_owned())
        .send()
        .await
        .unwrap();

    match response.status().as_u16() {
        200..=299 => {
            let body = response.text().await?;
            println!("Success! Body:\n{}", body);
        }
        400..=599 => {
            let status = response.status();
            let error_message = response.text().await?;
            println!("Error {}: {}", status, error_message);
        }
        _ => {
            println!("Unexpected status code: {}", response.status());
        }
    }    
    Ok(())
}

async fn hello() -> Result<(), Error> {
    let url = format!("http://localhost:8000/api/hello");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status().as_u16() {
        200..=299 => {
            let body = response.text().await?;
            println!("Success! Body:\n{}", body);
        }
        400..=599 => {
            let status = response.status();
            let error_message = response.text().await?;
            println!("Error {}: {}", status, error_message);
        }
        _ => {
            println!("Unexpected status code: {}", response.status());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    post_device().await?;

    Ok(())
} 
