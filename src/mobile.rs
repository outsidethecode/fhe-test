#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::catch;
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use reqwest::{Error, header::ACCEPT, header::CONTENT_TYPE};
use secp256k1::{Secp256k1, SecretKey};
use rand::Rng;
use rocket_contrib::json::{Json, JsonValue, self};
use rocket::data::{FromDataSimple, FromData};
use rocket::http::{Status};
use rocket::request::Form;
use rocket::request::Request;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

#[macro_use]
extern crate lazy_static;

lazy_static::lazy_static! {
    static ref GLOBAL_DEVICES: Mutex<Vec<Device>> = Mutex::new(Vec::new());
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Call {
    agent_call_public_key: Vec<u8>,
    agent_encrypted_secret_key: Vec<u8>,
}

async fn generate_keypair() -> (secp256k1::SecretKey, secp256k1::PublicKey, Vec<u8>, Vec<u8>) {
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
    (secret_key, public_key, secret_key.secret_bytes().to_vec(), public_key.serialize().to_vec())
}

async fn post_device(device: Device) -> Result<(), Error> {
    let new_device_json = serde_json::to_string(&device).unwrap();

    let url = format!("http://localhost:8000/api/device");
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

#[post("/call", format = "json", data = "<call>")]
fn new_call(call: Json<Call>) -> Json<Call> {
    // let device: Device = device.into_inner();
    // let mut dummy_db: Vec<Device> = Vec::new();
    // dummy_db.push(device);
    // format!("Device added successfully: {:?}", dummy_db)

    // let new_device = Device {
    //     fhe_public_key: "String".to_string(),
    //     fmc_code: "String".to_string(),
    //     mobile_hash: "String".to_string()
    // };
    // Json(new_device)

    call
}

static LOG_LEVEL: u8 = 0;

#[tokio::main]
async fn main() -> Result<(), Error> {

    rocket::ignite()
    .mount("/api", routes![new_call])
    .launch();

    let (call_private_key, call_public_key, call_serialized_private_key, call_serialized_public_key) = generate_keypair().await;
    let (encryption_private_key, encryption_public_key, encryption_serialized_private_key, encryption_serialized_public_key) = generate_keypair().await;

    let device = Device {
        call_public_key: call_serialized_public_key,
        encryption_public_key: encryption_serialized_public_key,
        fmc_code: "String1111".to_string(),
        mobile_hash: "String1111".to_string()
    };


    let mut devices = GLOBAL_DEVICES.lock().unwrap();
    devices.push(device.clone());

    post_device(device).await?;

    Ok(())
} 
