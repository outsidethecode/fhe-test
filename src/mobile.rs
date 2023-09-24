
use rocket::http::ContentType;
use serde::{Deserialize, Serialize};
use reqwest::{Error, header::ACCEPT, header::CONTENT_TYPE};
use secp256k1::{Secp256k1, SecretKey};
use rand::Rng;
use rocket_contrib::json::{Json, JsonValue, self};
use sunscreen::{
    fhe_program,
    types::{bfv::Signed, Cipher},
    Compiler, FheRuntime, PublicKey, PrivateKey,
};

use flate2::Compression;
use flate2::write::GzEncoder;


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Device {
    ec_public_key: Vec<u8>,
    fhe_public_key: Vec<u8>,
    encrypted_secret_key: Vec<Vec<u8>>,
    fmc_code: String,
    mobile_hash: String
}

#[fhe_program(scheme = "bfv")]
fn simple_add(a: Cipher<Signed>, b: Cipher<Signed>) -> Cipher<Signed> {
    a + b
}

fn calc_slice_value(byte_array: &[u8]) -> u32 {
    let mut value: u32 = 0u32;
    
    for &byte in byte_array.iter() {
        value = (value << 8) | u32::from(byte);
    }
    
    value
}

fn u8_32_array_to_u32_8_array(arr: &[u8; 32]) -> [u32; 8] {
    let mut result: [u32; 8] = [0; 8];

    for i in 0..8 {
        let sub_arr = &arr[i*4 .. (i+1)*4];
        result[i] = calc_slice_value(sub_arr);
    }

    result
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

async fn generate_fhe_keypair() -> (PrivateKey, PublicKey, Vec<u8>, Vec<u8>) {
    let app = Compiler::new()
    .fhe_program(simple_add)
    .compile().unwrap();
    let runtime = FheRuntime::new(app.params()).unwrap();

    let (public_key, private_key) = runtime.generate_keys().unwrap();
    let serialized_private_key = serde_json::to_vec(&private_key).expect("Serialization failed");
    let serialized_public_key = serde_json::to_vec(&public_key).expect("Serialization failed");
    (private_key, public_key, serialized_private_key, serialized_public_key)
}

async fn post_device() -> Result<(), Error> {

    let (ec_private_key, ec_public_key, ec_serialized_private_key, ec_serialized_public_key) = generate_keypair().await;
    let (fhe_private_key, fhe_public_key, fhe_serialized_private_key, fhe_serialized_public_key) = generate_fhe_keypair().await;
    let mut encoder = GzEncoder::new(fhe_serialized_public_key, Compression::best());
    let compressed_fhe_public_key = encoder.finish().unwrap();

    let app = Compiler::new()
    .fhe_program(simple_add)
    .compile().unwrap();
    let runtime = FheRuntime::new(app.params()).unwrap();

    let mut encrypted_secret_key = Vec::new();
    let compressed_ec_private_key = u8_32_array_to_u32_8_array(&ec_private_key.secret_bytes());
    for (i, chunk) in compressed_ec_private_key.iter().enumerate() {
        let i64_value : i64 = *chunk as i64;
        println!("------------------------------------------------------\n\n");

        println!("{:?}", serde_json::to_vec(&runtime.encrypt(Signed::from(i64_value), &fhe_public_key).unwrap()).unwrap());
        encrypted_secret_key.push(serde_json::to_vec(&runtime.encrypt(Signed::from(i64_value), &fhe_public_key).unwrap()).unwrap());
        println!("------------------------------------------------------\n\n");

    }

    let new_device = Device {
        ec_public_key: ec_serialized_public_key,
        fhe_public_key: compressed_fhe_public_key,
        encrypted_secret_key: encrypted_secret_key,
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
