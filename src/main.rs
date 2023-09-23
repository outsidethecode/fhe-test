#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use secp256k1::{Secp256k1};
use sunscreen::{Ciphertext, PublicKey};
use tfhe::{prelude::*, FheUint16};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, CompactPublicKey};
use ecies::{decrypt, encrypt, utils::generate_keypair};
use std::str::FromStr;
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

use rocket::*;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::json::Json;

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


#[derive(Deserialize)]
struct Device {
    fhe_public_key: String,
    fmc_code: String,
    mobile_hash: String
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{\"status\": \"success\", \"message\": \"Hello API!\"}")
}

#[post("/device", data = "<device>")]
fn new_device(device: Json<Device>) -> String {
    // let device: Device = device.into_inner();
    let mut dummy_db: Vec<Device> = Vec::new();
    dummy_db.push(device);
    format!("Device added successfully: {:?}", dummy_db)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    rocket::ignite()
    .register(catchers![not_found])
    .mount("/api", routes![hello, new_device])
    .launch();
    

    // Initialize the secp256k1 context
    let secp = Secp256k1::new();

    // Generate two random private keys
    let mut rng = rand::thread_rng();
    let mut sk1_bytes: [u8; 32] = rng.gen();
    for i in 0..1 {
        sk1_bytes[i] &= 0b01111111;
    }
    let mut sk2_bytes: [u8; 32] = rng.gen();
    for i in 0..1 {
        sk2_bytes[i] &= 0b01111111;
    }

    // Create SecretKey objects from the generated private key bytes
    let sk1 = secp256k1::SecretKey::from_slice(&sk1_bytes).expect("Invalid private key");
    let sk2 = secp256k1::SecretKey::from_slice(&sk2_bytes).expect("Invalid private key");

    print!("sk1 {:?}\n", sk1);
    print!("sk1 bytes {:?}\n", sk1.secret_bytes());
    print!("sk1 bytes {:?}\n", sk1_bytes);

    // Calculate the corresponding public keys for sk1 and sk2
    let pk1 = secp256k1::PublicKey::from_secret_key(&secp, &sk1);
    let pk2 = secp256k1::PublicKey::from_secret_key(&secp, &sk2);

    // Calculate the sum of public keys pk1 and pk2
    let sum_public_key = pk1.combine(&pk2).expect("Failed to add public keys");

    print!("pk1 {:?}\n", pk1);
    print!("pk2 {:?}\n", pk2);
    print!("sum_public_key {:?}\n", sum_public_key);

    // Calculate the sum of private keys sk1 and sk2
    let mut sk_sum_bytes = add_arrayz(&sk1_bytes, &sk2_bytes);
    let sk = secp256k1::SecretKey::from_slice(&sk_sum_bytes).expect("Invalid private key");
    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);

    print!("public_key {:?}\n", pk);

    // // Calculate the corresponding public key for the sum of private keys
    // let sk_sum = SecretKey::from_slice(&sk_sum_bytes).expect("Invalid private key");
    // let sum_public_key_from_sk_sum = PublicKey::from_secret_key(&secp, &sk_sum);

    // println!("Public Key from Point Addition: {:?}", sum_public_key.serialize_compressed());
    // println!("Public Key from Sum of Private Keys: {:?}", sum_public_key_from_sk_sum.serialize_compressed());

    // // Check if the two public keys are equal
    // if sum_public_key == sum_public_key_from_sk_sum {
    //     println!("The public keys are equal.");
    // } else {
    //     println!("The public keys are not equal.");
    // }
    // let app = Compiler::new()
    // .fhe_program(simple_add)
    // .compile()?;

    // let runtime = FheRuntime::new(app.params())?;
    // let (public_key, private_key) = runtime.generate_keys()?;
    // let serialized_public_key = serde_json::to_vec(&public_key).expect("Serialization failed");
    // let deserialized_public_key: PublicKey = serde_json::from_slice(&serialized_public_key).expect("Deserialization failed");


    // let mut rng = rand::thread_rng();

    // let mut sk1_bytes: [u8; 32] = rng.gen();
    // for i in 0..1 {
    //     sk1_bytes[i] = 0;
    // }

    // let sk1 = libsecp256k1::SecretKey::parse_slice(&sk1_bytes).expect("Invalid secret key");   
    // let pk1 = libsecp256k1::PublicKey::from_secret_key(&sk1);
    // let (sk1, pk1) = (&sk1.serialize(), &pk1.serialize());
    // print!("sk1 {:?}\n", sk1);
    // print!("sk1 value {:?}\n", calculate_value(sk1));

    // let mut sk2_bytes: [u8; 32] = rng.gen();
    // for i in 0..1 {
    //     sk2_bytes[i] = 0;
    // }
    // let sk2 = libsecp256k1::SecretKey::parse_slice(&sk2_bytes).expect("Invalid secret key");   
    // let pk2 = libsecp256k1::PublicKey::from_secret_key(&sk2);
    // let (sk2, pk2) = (&sk2.serialize(), &pk2.serialize());
    // print!("sk2 {:?}\n", sk2);
    // print!("sk2 value {:?}\n", calculate_value(sk2));

    // let sk_bytes: [u8; 64] = add_private_keys(&sk1, &sk2);
    // let sk = libsecp256k1::SecretKey::parse_slice(&sk_bytes[0..32]).expect("Invalid secret key");   
    // let pk = libsecp256k1::PublicKey::from_secret_key(&sk);
    // let (sk, pk) = (&sk.serialize(), &pk.serialize());
    // let pk_value = calculate_pk_value(pk);
    // let pk_value_2 = calculate_pk_value_2(&add_public_keys(pk1, pk2));

    // print!("sk {:?}\n", sk);
    // print!("sk value {:?}\n", calculate_value(sk));
    // print!("sk1 + sk2 ---> {:?}\n", calculate_value(sk1) + calculate_value(sk2));
    // print!("PK1 value {:?}\n", pk1);
    // print!("PK2 value {:?}\n", pk2);
    // print!("PK1 + PK2  {:?}\n", add_public_keys(pk1, pk2));

    // print!("PK value {:?}\n", pk_value);
    // print!("PK value {:?}\n", pk);
    // print!("PK value 2 {:?}\n", pk_value_2);
    // print!("PK value 3 {:?}\n", calculate_pk_value(pk1) + calculate_pk_value(pk2));
  

    // let start = Instant::now();

    // let mut c1 = Vec::new();
    // let compressed_sk1 = u8_32_array_to_u32_8_array(&sk1);
    // for (i, chunk) in compressed_sk1.iter().enumerate() {
    //     let i64_value : i64 = *chunk as i64;
    //     c1.push(runtime.encrypt(Signed::from(i64_value), &deserialized_public_key)?);
    // }

    // let mut c2 = Vec::new();
    // let compressed_sk2 = u8_32_array_to_u32_8_array(&sk2);
    // for (i, chunk) in compressed_sk2.iter().enumerate() {
    //     let i64_value : i64 = *chunk as i64;
    //     c2.push(runtime.encrypt(Signed::from(i64_value), &deserialized_public_key)?);
    // }

    // let mut c = Vec::new();
    // for i in 0..8 {
    //     let a = c1[i].clone();
    //     let b = c2[i].clone();
    //     c.push(runtime.run(app.get_fhe_program(simple_add).unwrap(), vec![a, b], &deserialized_public_key)?);
    // }
  
    // let mut c_value: BigUint = BigUint::from(0u128);
    // for element in c.iter() {
    //     let decrypted_ci : Signed = runtime.decrypt(&element[0], &private_key)?;
    //     c_value <<= 32;
    //     c_value += BigUint::from_str(&decrypted_ci.to_string()).unwrap();
    // }

    // let c_value_bytes = get_c_value_bytes(c_value);
    // let secret_key = libsecp256k1::SecretKey::parse_slice(&c_value_bytes).expect("Invalid secret key");   
    // let pkz2 = libsecp256k1::PublicKey::from_secret_key(&secret_key);
    // let pkz2_serialized = pkz2.serialize();

    // println!("PKz2 {:?}", &pkz2_serialized);
    // println!("PKz2 value 1 {:?}", BigUint::from_bytes_be(&pkz2_serialized));
    // println!("PKz2 value 2 {:?}", calculate_pk_value(&pkz2_serialized));

    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

 

    Ok(())
}