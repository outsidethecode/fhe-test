#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::catch;
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use reqwest::{Error, header::ACCEPT, header::CONTENT_TYPE};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use rand::Rng;
use rocket_contrib::json::{Json, JsonValue, self};
use rocket::data::{FromDataSimple, FromData};
use rocket::http::{Status};
use rocket::request::Form;
use rocket::request::Request;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use ecies::{decrypt, encrypt, utils::generate_keypair};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Call {
    random_devices: Vec<Device>,
    agent: Agent
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Agent {
    agent_public_key: Vec<u8>,
    encrypted_agent_secret_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Device {
    call_public_key: Vec<u8>,
    encryption_public_key: Vec<u8>,
    fmc_code: String,
    mobile_hash: String
}

impl Device {
    // Define a new method as an associated function
    fn new() -> Self {
        Device { 
            call_public_key: Vec::new(),
            encryption_public_key: Vec::new(),
            fmc_code: "".to_string(),
            mobile_hash: "".to_string()
        }
    }

    fn clone(&self) -> Self {
        Device {
            call_public_key: self.call_public_key.clone(),
            encryption_public_key: self.encryption_public_key.clone(),
            fmc_code: self.fmc_code.clone(),
            mobile_hash: self.mobile_hash.clone()
        }
    }
}

async fn generate_key_pair() -> (secp256k1::SecretKey, secp256k1::PublicKey) {
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
    (secret_key, public_key)
}

async fn call_device(call: Call) -> Result<(), Error> {
    println!("---------->>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    let call_json = serde_json::to_string(&call).unwrap();
    let url = format!("http://localhost:8000/api/call");
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .body(call_json)
        .send()
        .await
        .unwrap();

    match response.status().as_u16() {
        200..=299 => {
            let body = response.text().await?;
            println!("Request sent successfully! Body:\n{}", body);
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
    println!("----------<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<");

    Ok(())
}

async fn get_devices() -> Result<(), Error> {
    let url = format!("http://localhost:8000/api/devices");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    match response.status().as_u16() {
        200..=299 => {
            let devices: Vec<Device> = response.json().await?;
            println!("Received devices:\n{:?}", devices);

            let (agent_secret_key, agent_public_key) = generate_key_pair().await;
            let last_device = devices.last().unwrap();

            let device_encryption_public_key: &[u8] = &last_device.encryption_public_key;
            let recipient_device_public_key = PublicKey::from_slice(
                device_encryption_public_key
            ).expect("Invalid public key");

            let encrypted_agent_secret_key = encrypt(&recipient_device_public_key.serialize(), &agent_secret_key.secret_bytes()).unwrap();

            let agent: Agent = Agent {
                agent_public_key: agent_public_key.serialize().to_vec(),
                encrypted_agent_secret_key: encrypted_agent_secret_key,
            };
            
            let call = Call {
                random_devices: devices,
                agent: agent
            };
            println!("{:?}", call);
            call_device(call).await?;

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

    get_devices().await?;

    Ok(())
} 
