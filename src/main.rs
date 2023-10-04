// #![feature(decl_macro)]
// #[macro_use]
// extern crate rocket;

// use diesel::result::Error;
// use ecies::{decrypt, encrypt, utils::generate_keypair};
// use lacore::models::NewDevice;
// use lacore::{schema, establish_connection, create_device, get_devices, models};
// use libsecp256k1;
// use num_bigint::BigUint;
// use rand::Rng;
// use reqwest::{header::ACCEPT, header::CONTENT_TYPE};
// use rocket::data::{FromData, FromDataSimple};
// use rocket::http::{ContentType, Status};
// use secp256k1::Secp256k1;
// use serde::{Deserialize, Serialize};
// use std::str::FromStr;
// use std::{thread, env};
// use std::time::{Duration, Instant};
// use sunscreen::{
//     fhe_program,
//     types::{bfv::Signed, Cipher},
//     Compiler, FheRuntime,
// };
// use sunscreen::{Ciphertext, PublicKey};
// use tfhe::{generate_keys, set_server_key, CompactPublicKey, ConfigBuilder, FheUint32};
// use tfhe::{prelude::*, FheUint16};
// use tokio::runtime::Runtime;

// use rocket::request::Form;
// use rocket::*;
// use rocket_contrib::json::{self, Json, JsonValue};

// use postgres::{Client, NoTls};
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use tokio::spawn;
// use rocket::response::status::{Custom, self};
// use rocket::response::{Responder, Response};


// #[fhe_program(scheme = "bfv")]
// fn simple_add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
//     a + b
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// struct Device {
//     message_encryption_public_key: Vec<u8>,
//     pke_public_key: Vec<u8>,
//     fcm_code: String,
//     device_hash: String,
// }

// impl Device {
//     // Define a new method as an associated function
//     fn new() -> Self {
//         Device {
//             message_encryption_public_key: Vec::new(),
//             pke_public_key: Vec::new(),
//             fcm_code: "".to_string(),
//             device_hash: "".to_string(),
//         }
//     }

//     fn clone(&self) -> Self {
//         Device {
//             message_encryption_public_key: self.message_encryption_public_key.clone(),
//             pke_public_key: self.pke_public_key.clone(),
//             fcm_code: self.fcm_code.clone(),
//             device_hash: self.device_hash.clone(),
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
// struct Call {
//     random_devices: Vec<Device>,
//     agent: Agent,
// }

// impl Call {
//     fn clone(&self) -> Self {
//         Call {
//             random_devices: self.random_devices.clone(),
//             agent: self.agent.clone(),
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// struct DeviceFilter {
//     device_hashes: Vec<String>
// }

// impl DeviceFilter {
//     fn clone(&self) -> Self {
//         DeviceFilter {
//             device_hashes: self.device_hashes.clone(),
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
// struct Agent {
//     agent_public_key: Vec<u8>,
//     encrypted_agent_secret_key: Vec<u8>,
// }

// impl Agent {
//     fn clone(&self) -> Self {
//         Agent {
//             agent_public_key: self.agent_public_key.clone(),
//             encrypted_agent_secret_key: self.encrypted_agent_secret_key.clone(),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum AppError {
//     InvalidParameters,
//     // Add more error variants as needed.
// }

// #[derive(Serialize)]
// struct ErrorResponse {
//     error: String,
// }

// impl AppError {
//     fn status(&self) -> Status {
//         match self {
//             AppError::InvalidParameters => Status::BadRequest,
//             // Handle other error variants here.
//         }
//     }

//     fn to_json_response(&self) -> Response<'static> {
//         let status = self.status();
//         let json_error = ErrorResponse {
//             error: format!("{:?}", self),
//         };
//         Response::build()
//             .status(status)
//             .header(ContentType::JSON)
//             .sized_body(std::io::Cursor::new(serde_json::to_string(&json_error).unwrap()))
//             .finalize()
//     }
// }

// fn host() -> String {
//     env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
// }

// fn port() -> String {
//     env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
// }

// fn device_created(device: models::Device) -> status::Created<Json<models::Device>> {
//     status::Created(
//         format!("{host}:{port}/post/{id}", host = host(), port = port(), id = device.id).to_string(),
//         Some(Json(device)))
// }

// fn error_status(error: Error) -> Status {
//     match error {
//         Error::NotFound => Status::NotFound,
//         _ => Status::InternalServerError
//     }
// }

// #[post("/devices", format = "application/json", data = "<new_device>")]
// fn register_device(new_device: Json<NewDevice>) -> Result<status::Created<Json<models::Device>>, Status> {
//     // let new_device = Device {
//     //     message_encryption_public_key: device.message_encryption_public_key.clone(),
//     //     pke_public_key: device.pke_public_key.clone(),
//     //     fcm_code: device.fcm_code.clone(),
//     //     device_hash: device.device_hash.clone(),
//     // };
//     let connection = &mut establish_connection();
//     create_device(connection, &new_device.message_encryption_public_key, &new_device.pke_public_key, &new_device.fcm_code, &new_device.device_hash)
//     .map(|device| device_created(device))
//     .map_err(|error| error_status(error));

// }

// #[post("/random_devices", format = "json", data = "<device_filter>")]
// fn get_random_devices(device_filter: Json<DeviceFilter>) -> Json<Vec<Device>> {
//     let connection = &mut establish_connection();
//     let mut all_devices = Vec::new();
    
//     for row in get_devices(connection, device_filter.device_hashes.clone()) {
//         let device = Device {
//             message_encryption_public_key: row.message_encryption_public_key.unwrap(), 
//             pke_public_key: row.pke_public_key.unwrap(), 
//             fcm_code: row.fcm_code.unwrap(), 
//             device_hash: row.device_hash.unwrap() 
//         };
//         all_devices.push(device.clone());
//     }

//     Json(all_devices)
// }

// #[post("/call", format = "json", data = "<call>")]
// fn new_call(call: Json<Call>) -> Json<Call> {
//     let call_clone = call.clone();
//     let devices = call_clone.random_devices;
//     // let mut handles = vec![];

//     println!("Rand Devices {:?}", devices);
//     for device in &devices {
//         let call_clone = call.clone(); // Clone call for each thread
//         let device_clone = device.clone(); // Clone the device for each thread

//         let rt = Runtime::new().unwrap();
//         rt.block_on(async move {
//             push_notification(&device_clone, &call_clone.agent).await;
//         });
//     }

//     call
// }

// #[catch(404)]
// fn not_found(req: &Request) -> String {
//     format!("Oh no! We couldn't find the requested path '{}'", req.uri())
// }

// async fn push_notification(device: &Device, agent: &Agent) -> Result<(), reqwest::Error> {
//     let agent = serde_json::to_string(&agent).unwrap();

//     let url = format!("http://localhost:8001/api/call");
//     let client = reqwest::Client::new();
//     let response = client
//         .post(url)
//         .header(ACCEPT, "application/json")
//         .header(CONTENT_TYPE, "application/json")
//         .body(agent)
//         .send()
//         .await
//         .unwrap();

//     match response.status().as_u16() {
//         200..=299 => {
//             let body = response.text().await?;
//             println!("Push notification sent! Body:\n{}", body);
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

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     rocket::ignite()
//         .register(catchers![not_found])
//         .mount("/api", routes![register_device, get_random_devices, new_call])
//         .launch();

//     Ok(())
// }



#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;

mod core;
mod schema;
mod connection;

fn main() {
    dotenv().ok();
    core::router::create_routes();
}


