#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::config::Environment;
use rocket::{catch, Config};
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
use ecies::{decrypt, encrypt, utils::generate_keypair};


// // #[macro_use]
// // extern crate lazy_static;

// lazy_static! {
//     static ref GLOBAL_DEVICES: Arc<Mutex<Vec<MyDevice>>> = Arc::new(Mutex::new(Vec::<MyDevice>::new()));
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct Device {
//     message_encryption_public_key: Vec<u8>,
//     pke_public_key: Vec<u8>,
//     fcm_code: String,
//     device_hash: String
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct MyDevice {
//     message_encryption_secret_key: Vec<u8>,
//     message_encryption_public_key: Vec<u8>,
//     pke_secret_key: Vec<u8>,
//     pke_public_key: Vec<u8>,
//     fcm_code: String,
//     device_hash: String
// }

// impl Device {
//     // Define a new method as an associated function
//     fn new() -> Self {
//         Device {
//             message_encryption_public_key: Vec::new(),
//             pke_public_key: Vec::new(),
//             fcm_code: "".to_string(),
//             device_hash: "".to_string()
//         }
//     }

//     fn clone(&self) -> Self {
//         Device {
//             message_encryption_public_key: self.message_encryption_public_key.clone(),
//             pke_public_key: self.pke_public_key.clone(),
//             fcm_code: self.fcm_code.clone(),
//             device_hash: self.device_hash.clone()
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct Call {
//     random_devices: Vec<Device>,
//     agent: Agent
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct Agent {
//     agent_public_key: Vec<u8>,
//     encrypted_agent_secret_key: Vec<u8>,
// }

// async fn generate_key_pair() -> (secp256k1::SecretKey, secp256k1::PublicKey) {
//     // Initialize the secp256k1 context
//     let secp = Secp256k1::new();
    
//     // Generate two random private keys
//     let mut rng = rand::thread_rng();
//     let mut secret_key_bytes: [u8; 32] = rng.gen();
//     for i in 0..1 {
//         secret_key_bytes[i] &= 0b01111111;
//     }

//     // Create SecretKey objects from the generated private key bytes
//     let secret_key = secp256k1::SecretKey::from_slice(&secret_key_bytes).expect("Invalid private key");
//     let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
//     (secret_key, public_key)
// }

// async fn register_device() -> Result<(), Error> {
//     let my_devices = GLOBAL_DEVICES.lock().unwrap();
//     let my_device = my_devices.get(my_devices.len()-1).unwrap();
//     let device = Device {
//         message_encryption_public_key: my_device.message_encryption_public_key.clone(),
//         pke_public_key: my_device.pke_public_key.clone(),
//         fcm_code: my_device.fcm_code.clone(),
//         device_hash: my_device.device_hash.clone()
//     };

//     let new_device_json = serde_json::to_string(&device).unwrap();

//     let url = format!("http://localhost:8000/api/devices");
//     let client = reqwest::Client::new();
//     let response = client
//         .post(url)
//         .header(ACCEPT, "application/json")
//         .header(CONTENT_TYPE, "application/json")
//         .body(new_device_json)
//         .send()
//         .await
//         .unwrap();

//     match response.status().as_u16() {
//         200..=299 => {
//             let body = response.text().await?;
//             println!("Success! Body:\n{}", body);
//         }
//         400..=599 => {
//             let status = response.status();
//             let error_message = response.text().await?;
//             println!("Error {}: {}", status, error_message);
//         }
//         _ => {
//             println!("Unexpected status code: {}", response.status());
//         }
//     }    
//     Ok(())
// }

// #[post("/call", format = "json", data = "<agent>")]
// fn new_call(agent: Json<Agent>) -> Json<Agent> {
//     // Initialize the secp256k1 context
//     let secp = Secp256k1::new();
//     let devices = GLOBAL_DEVICES.lock().unwrap();
//     let my_device = devices.get(devices.len()-1).unwrap();
//     let my_device_pke_public_key: secp256k1::PublicKey = secp256k1::PublicKey::from_slice(my_device.pke_public_key.as_slice()).unwrap();
//     let agent_public_key: secp256k1::PublicKey = secp256k1::PublicKey::from_slice(agent.agent_public_key.as_slice()).unwrap();
//     let sum_public_key: secp256k1::PublicKey = my_device_pke_public_key.combine(&agent_public_key).expect("Failed to add public keys");

//     let pke_secret_key = secp256k1::SecretKey::from_slice(my_device.pke_secret_key.as_slice()).unwrap().secret_bytes();
//     match decrypt(&pke_secret_key, &agent.encrypted_agent_secret_key) {
//         Ok(agent_secret_key) => {
//             let mut agent_secret_key_array: [u8; 32] = [0; 32];
//             agent_secret_key_array.copy_from_slice(agent_secret_key.as_slice());
//             let sum_secret_keys = add_arrays(&agent_secret_key_array, &pke_secret_key);
//             let secret_key = secp256k1::SecretKey::from_slice(&sum_secret_keys).expect("Invalid private key");
//             let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

//             if sum_public_key.eq(&public_key) {
//                 println!("This is mine!");
//             } else {
//                 println!("Decrypt error: Not for me");
//             }
//         }
//         Err(err) => {
//             println!("Err: Not for me");
//         }
//     }

//     agent
// }

// fn add_arrays(arr1: &[u8; 32], arr2: &[u8; 32]) -> [u8; 32] {
//     let mut result: [u8; 32] = [0; 32];
//     let mut carry: u8 = 0;

//     for i in (0..32).rev() {
//         let sum: u32 = arr1[i] as u32 + arr2[i] as u32 + carry as u32;
//         result[i] = sum as u8 ;
//         carry = if sum > 255 { 1 } else { 0 };
//     }
//     result
// }

// async fn create_new_device() {
//     let (message_encryption_secret_key, message_encryption_public_key) = generate_key_pair().await;
//     let (pke_secret_key, pke_public_key) = generate_key_pair().await;

//     let my_device = MyDevice {
//         message_encryption_secret_key: message_encryption_secret_key.secret_bytes().to_vec(),
//         message_encryption_public_key: message_encryption_public_key.serialize().to_vec(),
//         pke_secret_key: pke_secret_key.secret_bytes().to_vec(),
//         pke_public_key: pke_public_key.serialize().to_vec(),
//         fcm_code: "String1111".to_string(),
//         device_hash: "String1111".to_string()
//     };

//     let mut devices = GLOBAL_DEVICES.lock().unwrap();
//     devices.push(my_device);
// }

// #[tokio::main]
fn main() -> Result<(), Error> {
    // for i in 1..10 {
    //     create_new_device().await;
    //     register_device().await?;
    // }

    // let config = Config::build(Environment::Development)
    // .address("localhost")
    // .port(8001)
    // .finalize().unwrap();

    // rocket::custom(config)
    // .mount("/api", routes![new_call])
    // .launch();

    Ok(())
} 
