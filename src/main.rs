use rsa::signature::digest::typenum::Unsigned;
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
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use crypto_box::{
    aead::{Aead, AeadCore, OsRng},
    SalsaBox, SecretKey
};


use sunscreen::{
    fhe_program,
    types::{bfv::Signed, Cipher},
    Compiler, Error, FheRuntime,
};

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
        println!("{}", arr1[i]);
        println!("{}", arr2[i]);
        println!("{}", carry);
                 

        let sum: u32 = arr1[i] as u32 + arr2[i] as u32 + carry as u32;
        result[i] = sum as u8 ;
        println!("{}", sum);
        // Calculate carry for the next iteration
        carry = if sum > 255 { 1 } else { 0 };
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



fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    // // Initialize the secp256k1 context
    // let secp = Secp256k1::new();

    // // Replace these private keys with your actual private keys
    // let private_key1_bytes: [u8; 32] = [0x01; 32];
    // let private_key2_bytes: [u8; 32] = [0x02; 32];
    // let private_key_bytes: [u8; 64] = [0x02; 64];

    // // Create SecretKey objects from the private key bytes
    // let private_key1 = SecretKey::from_slice(&private_key1_bytes).expect("Invalid private key");
    // let private_key2 = SecretKey::from_slice(&private_key2_bytes).expect("Invalid private key");

    // // Calculate the corresponding public keys for the private keys
    // let public_key1 = secp256k1::PublicKey::from_secret_key(&secp, &private_key1_bytes);
    // let public_key2 = secp256k1::PublicKey::from_secret_key(&secp, &private_key2_bytes);

    

    // println!("Public Key 1: {:?}", public_key1.serialize_compressed());
    // println!("Public Key 2: {:?}", public_key2.serialize_compressed());
    // println!("Sum Public Key: {:?}", sum_public_key.serialize_compressed());

    // 




    // let mut rng = rand::thread_rng();
    
    // // Generate random test data for arr1 and arr2
    // let mut arr1: [u8; 32] = rng.gen();
    // let mut arr2: [u8; 32] = rng.gen();

    // for i in 0..31 {
    //     arr1[i] = 0;
    //     arr2[i] = 0;
    // }


    // println!("arr1: {:?}", arr1);
    // println!("arr2: {:?}", arr2);

    // let sum = add_arrayz(&arr1, &arr2);

    // println!("Sum: {:?}", sum);


    let app = Compiler::new()
    .fhe_program(simple_add)
    .compile()?;

    let runtime = FheRuntime::new(app.params())?;

    let (public_key, private_key) = runtime.generate_keys()?;

    let serialized_public_key = serde_json::to_vec(&public_key).expect("Serialization failed");

    let deserialized_public_key: PublicKey = serde_json::from_slice(&serialized_public_key).expect("Deserialization failed");


    let a = runtime.encrypt(Signed::from(2147483647), &deserialized_public_key)?;
    // let b = runtime.encrypt(Signed::from(2147483647), &public_key)?;

    // let start = Instant::now();

    // let results = runtime.run(app.get_fhe_program(simple_add).unwrap(), vec![a, b], &public_key)?;
    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    // let c: Signed = runtime.decrypt(&results[0], &private_key)?;

    // println!("{}", c);
    // Ok(())

    let mut rng = rand::thread_rng();

    let mut sk1_bytes: [u8; 32] = rng.gen();
    for i in 0..31 {
        sk1_bytes[i] = 0;
    }

    let sk1 = libsecp256k1::SecretKey::parse_slice(&sk1_bytes).expect("Invalid secret key");   
    let pk1 = libsecp256k1::PublicKey::from_secret_key(&sk1);
    let (sk1, pk1) = (&sk1.serialize(), &pk1.serialize());
    print!("sk1 {:?}\n", sk1);
    print!("sk1 value {:?}\n", calculate_value(sk1));

    let mut sk2_bytes: [u8; 32] = rng.gen();
    for i in 0..31 {
        sk2_bytes[i] = 0;
    }
    let sk2 = libsecp256k1::SecretKey::parse_slice(&sk2_bytes).expect("Invalid secret key");   
    let pk2 = libsecp256k1::PublicKey::from_secret_key(&sk2);
    let (sk2, pk2) = (&sk2.serialize(), &pk2.serialize());
    print!("sk2 {:?}\n", sk2);
    print!("sk2 value {:?}\n", calculate_value(sk2));

    let mut sk_bytes: [u8; 64] = add_private_keys(&sk1, &sk2);
    let sk = libsecp256k1::SecretKey::parse_slice(&sk_bytes[0..32]).expect("Invalid secret key");   
    let pk = libsecp256k1::PublicKey::from_secret_key(&sk);
    let (sk, pk) = (&sk.serialize(), &pk.serialize());
    let pk_value = calculate_pk_value(pk);

    print!("sk {:?}\n", sk);
    print!("sk value {:?}\n", calculate_value(sk));
    print!("sk1 + sk2 ---> {:?}\n", calculate_value(sk1) + calculate_value(sk2));

    // let pk1_value = calculate_pk_value(pk1);
    // let pk2_value = calculate_pk_value(pk2);
    // print!("PK1 value {:?}\n", pk1_value);
    // print!("PK2 value {:?}\n", pk2_value);
    print!("PK value {:?}\n", pk_value);
  
    // print!("sk bytes {:?}\n", &sk_bytes);
    // print!("sk bytes 2 {:?}\n", &sk_bytes_2);
    // print!("sk1 + sk2 : 22222 {:?}\n", calculate_value(&sk_bytes));
    // print!("sk1 + sk2 : 33333 {:?}\n", calculate_value(&sk_bytes_2));

    let start = Instant::now();

    let mut c1 = Vec::new();
    let compressed_sk1 = u8_32_array_to_u32_8_array(&sk1);
    for (i, chunk) in compressed_sk1.iter().enumerate() {
        let i64_value : i64 = *chunk as i64;
        c1.push(runtime.encrypt(Signed::from(i64_value), &deserialized_public_key)?);
    }

    let mut c2 = Vec::new();
    let compressed_sk2 = u8_32_array_to_u32_8_array(&sk2);
    for (i, chunk) in compressed_sk2.iter().enumerate() {
        let i64_value : i64 = *chunk as i64;
        c2.push(runtime.encrypt(Signed::from(i64_value), &deserialized_public_key)?);
    }

    let mut c = Vec::new();
    for i in 0..8 {
        let a = c1[i].clone();
        let b = c2[i].clone();
        c.push(runtime.run(app.get_fhe_program(simple_add).unwrap(), vec![a, b], &deserialized_public_key)?);
    }
  
    let mut c_value: BigUint = BigUint::from(0u128);
    for element in c.iter() {
        let decrypted_ci : Signed = runtime.decrypt(&element[0], &private_key)?;
        c_value <<= 32;
        c_value += BigUint::from_str(&decrypted_ci.to_string()).unwrap();
    }


    let c_value_bytes = c_value.to_bytes_be();
   
    let mut sk_empty_32_bytes: [u8; 32] = [0; 32];
    println!("sk_empty_32_bytes --- {:?}", sk_empty_32_bytes.clone());
    sk_empty_32_bytes[(32-c_value_bytes.len())..32].copy_from_slice(&c_value_bytes[..c_value_bytes.len()]);

    println!("sk bytes after copy {:?}", sk_empty_32_bytes.clone());
    println!("Value of sk bytes {:?}", calculate_value(&sk_empty_32_bytes.clone()));


    let secret_key = libsecp256k1::SecretKey::parse_slice(&sk_empty_32_bytes).expect("Invalid secret key");   
    let pkz2 = libsecp256k1::PublicKey::from_secret_key(&secret_key);
    let pkz2_serialized = pkz2.serialize();

    println!("PKz2 {:?}", calculate_pk_value(&pkz2_serialized));
    // println!("PKz2 ... {:?}", BigUint::from_bytes_be(&pkz2_serialized));


    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

 

    Ok(())
}