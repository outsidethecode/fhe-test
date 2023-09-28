#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rocket::data::{FromDataSimple, FromData};
use rocket::http::{ContentType, Status};
use secp256k1::{Secp256k1};
use sunscreen::{Ciphertext, PublicKey};
use tfhe::{prelude::*, FheUint16};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, CompactPublicKey};
use ecies::{decrypt, encrypt, utils::generate_keypair};
use tokio::runtime::Runtime;
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};
use num_bigint::BigUint;
use serde::{Serialize, Deserialize};
use libsecp256k1;
use rand::Rng;
use sunscreen::{
    fhe_program,
    types::{bfv::Signed, Cipher},
    Compiler, Error, FheRuntime,
};
use reqwest::{header::ACCEPT, header::CONTENT_TYPE};

use rocket::*;
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue, self};

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use tokio::spawn;

#[macro_use]
extern crate lazy_static;

lazy_static::lazy_static! {
    static ref GLOBAL_DEVICES: Mutex<Vec<Device>> = Mutex::new(Vec::new());
}

#[fhe_program(scheme = "bfv")]
fn simple_add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
    a + b
}

fn u8_32_array_to_u32_8_array(arr: &[u8; 32]) -> [u32; 8] {
    let mut result: [u32; 8] = [0; 8];

    for i in 0..8 {
        let sub_arr = &arr[i*4 .. (i+1)*4];
        result[i] = calc_slice_value(sub_arr);
    }

    result
}

fn calculate_value(byte_array: &[u8; 32]) -> BigUint {
    let mut value: BigUint = BigUint::from(0u128);
    
    for &byte in byte_array.iter() {
        value = (value << 8) | BigUint::from(byte);
    }
    
    value
}

fn calculate_pk_value(byte_array: &[u8; 65]) -> BigUint {
    let mut value: BigUint = BigUint::from(0u128);
    
    for &byte in byte_array.iter() {
        value = (value << 8) | BigUint::from(byte);
    }
    
    value
}

fn calculate_pk_value_2(byte_array: &[u8; 65]) -> BigUint {
    let mut value: BigUint = BigUint::from(0u128);
    
    for &byte in byte_array.iter() {
        value = (value << 8) | BigUint::from(byte);
    }
    
    value
}

fn calc_slice_value(byte_array: &[u8]) -> u32 {
    let mut value: u32 = 0u32;
    
    for &byte in byte_array.iter() {
        value = (value << 8) | u32::from(byte);
    }
    
    value
}

fn calc_value(byte_array: Vec<u8>) -> BigUint {
    let mut value: BigUint = BigUint::from(0u128);
    
    for &byte in byte_array.iter() {
        value = (value << 8) | BigUint::from(byte);
    }
    
    value
}

fn add_arrays(arr1: &[u8; 32], arr2: &[u8; 32]) -> [u8; 32] {
    let mut result: [u8; 32] = [0; 32];
    let mut carry: u8 = 0;

    for i in (0..32) {
        let sum = arr1[i] + arr2[i] + carry;
        result[i] = sum;
        
        // Calculate carry for the next iteration
        carry = if sum > 255 { 1 } else { 0 };
    }

    result
}

fn add_arrays3(arr1: &[u8; 65], arr2: &[u8; 65]) -> [u8; 65] {
    let mut result: [u8; 65] = [0; 65];
    let mut carry: u8 = 0;

    for i in (0..65) {
        result[i] = arr1[i].wrapping_add(arr2[i]);
    }

    result
}


fn add_arrays2(arr1: &[u8; 32], arr2: &[u8; 32]) -> [u8; 32] {
    let mut result: [u8; 32] = [0; 32];
    let mut carry: u8 = 0;

    for i in (0..32) {
        result[i] = arr1[i].wrapping_add(arr2[i]);
    }

    result
}


fn add_arrayz(arr1: &[u8; 32], arr2: &[u8; 32]) -> [u8; 32] {
    let mut result: [u8; 32] = [0; 32];
    let mut carry: u8 = 0;

    for i in (0..32).rev() {
        // println!("{}", arr1[i]);
        // println!("{}", arr2[i]);
        // println!("{}", carry);
                 

        let sum: u32 = arr1[i] as u32 + arr2[i] as u32 + carry as u32;
        result[i] = sum as u8 ;
        // println!("{}", sum);
        // Calculate carry for the next iteration
        carry = if sum > 255 { 1 } else { 0 };
    }

    if carry > 0 {
        println!("***********************************************{}", carry);
    }
    result
}

fn add_private_keys(arr1: &[u8; 32], arr2: &[u8; 32]) -> [u8; 64] {
    let mut result: [u8; 64] = [0; 64];
    let mut carry: u8 = 0;

    for i in (0..32).rev() {
        let sum: u32 = arr1[i] as u32 + arr2[i] as u32 + carry as u32;
        result[i] = sum as u8 ;
        // Calculate carry for the next iteration
        carry = if sum > 255 { 1 } else { 0 };
    }

    if carry > 0 {
        result[33] = 1
    }
    result
}

fn add_public_keys(arr1: &[u8; 65], arr2: &[u8; 65]) -> [u8; 65] {
    let mut result: [u8; 65] = [0; 65];
    let mut carry: u8 = 0;

    for i in (0..65).rev() {
        let sum: u32 = arr1[i] as u32 + arr2[i] as u32 + carry as u32;
        result[i] = sum as u8 ;
        // Calculate carry for the next iteration
        carry = if sum > 255 { 1 } else { 0 };
    }

    if carry > 0 {
        println!("******************************************** {}", carry);
        //result[65] = 1
    }
    result
}

fn get_c_value_bytes(c_value: BigUint) -> [u8; 32] {
    let c_value_bytes = c_value.to_bytes_be();
    let mut sk_empty_32_bytes: [u8; 32] = [0; 32];
    sk_empty_32_bytes[(32-c_value_bytes.len())..32].copy_from_slice(&c_value_bytes[..c_value_bytes.len()]);
    sk_empty_32_bytes
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
    random_devices: Vec<Device>,
    agent: Agent
}

impl Call {
    fn clone(&self) -> Self {
        Call {
            random_devices: self.random_devices.clone(),
            agent: self.agent.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct Agent {
    agent_public_key: Vec<u8>,
    encrypted_agent_secret_key: Vec<u8>,
}

impl Agent {
    fn clone(&self) -> Self {
        Agent {
            agent_public_key: self.agent_public_key.clone(),
            encrypted_agent_secret_key: self.encrypted_agent_secret_key.clone(),
        }
    }
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{\"status\": \"success\", \"message\": \"Hello API!\"}")
}

#[post("/device", format = "json", data = "<device>")]
fn register_device(device: Json<Device>) -> Json<Device> {
    let new_device = Device {
        call_public_key: device.call_public_key.clone(),
        encryption_public_key: device.encryption_public_key.clone(),
        fmc_code: device.fmc_code.clone(),
        mobile_hash: device.mobile_hash.clone()
    };

    let mut devices = GLOBAL_DEVICES.lock().unwrap();
    devices.push(new_device);

    device
}

#[get("/devices")]
fn get_devices() -> Json<Vec<Device>> {
    let mut devices = GLOBAL_DEVICES.lock().unwrap();
    let mut all_devices = Vec::new();

    devices.iter().for_each(|device| {
        all_devices.push(device.clone());
    });

    Json(all_devices)
}

#[post("/call", format = "json", data = "<call>")]
fn new_call(call: Json<Call>) -> Json<Call> {
    println!("**************************************************");

    let call_clone = call.clone();
    let agent = call_clone.agent.clone();
    let devices = call_clone.random_devices;
    // let mut handles = vec![];

    println!("Rand Devices {:?}", devices);
    for device in &devices {
        let call_clone = call.clone(); // Clone call for each thread
        let device_clone = device.clone(); // Clone the device for each thread

        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            println!("hello from the async block");
            push_notification(&device_clone, &call_clone.agent).await;
            print!("Pushhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh");
        });
    }

    call
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

async fn push_notification(device: &Device, agent: &Agent) -> Result<(), reqwest::Error> {
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    let agent = serde_json::to_string(&agent).unwrap();

    let url = format!("http://localhost:8001/api/call");
        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .header(ACCEPT, "application/json")
            .header(CONTENT_TYPE, "application/json")
            .body(agent)
            .send()
            .await
            .unwrap();

        match response.status().as_u16() {
            200..=299 => {
                let body = response.text().await?;
                println!("Push notification sent! Body:\n{}", body);
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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    rocket::ignite()
    .register(catchers![not_found])
    .mount("/api", routes![hello, register_device, get_devices, new_call])
    .launch();
    
    Ok(())
}