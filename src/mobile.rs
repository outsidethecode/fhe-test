
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use reqwest::{Error, header::ACCEPT, header::CONTENT_TYPE};
use secp256k1::{Secp256k1};
use rand::Rng;
use rocket_contrib::json::{Json, JsonValue, self};


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Device {
    ec_public_key: Vec<u8>,
    fhe_public_key: Vec<u8>,
    fmc_code: String,
    mobile_hash: String
}

async fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    // Initialize the secp256k1 context
    let secp = Secp256k1::new();
    
    // Generate two random private keys
    let mut rng = rand::thread_rng();
    let mut secret_key_bytes: [u8; 32] = rng.gen();
    for i in 0..1 {
        secret_key_bytes[i] &= 0b01111111;
    }

    // Create SecretKey objects from the generated private key bytes
    let secret_key = secp256k1::SecretKey::from_slice(&secret_key_bytes).expect("Invalid private key");
    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    (secret_key.secret_bytes().to_vec(), public_key.serialize().to_vec())
}

async fn post_device() -> Result<(), Error> {

    let (secret_key, public_key) = generate_keypair().await;

    let new_device = Device {
        ec_public_key: public_key,
        fhe_public_key: secret_key,
        fmc_code: "String1111".to_string(),
        mobile_hash: "String1111".to_string()
    };
    
    let new_device_json = serde_json::to_string(&new_device).unwrap();

    let url = format!("http://localhost:8000/api/device");
    // the rest is the same as before!
    let json_data = r#"{"fhe_public_key": keypair, "fmc_code": "7890", "mobile_hash": "h12345"}"#;
    println!("{}", json_data);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(new_device_json)
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
