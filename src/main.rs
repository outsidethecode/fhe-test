use tfhe::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, FheUint8, CompactPublicKey};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
extern crate secp256k1;
use secp256k1::{SecretKey, PublicKey};
extern crate hex;
use byteorder::{BigEndian, ReadBytesExt}; // 1.2.7
use rand::prelude::*;
use rand::thread_rng;
use rand::Rng;
use ecies::{decrypt, encrypt, utils::generate_keypair};
use std::time::{Duration, Instant};
use num_bigint::BigUint;

fn to_chunks(number: u64, width: u32, chunk_size: u32) -> Vec<u64> {
    assert!(width % chunk_size == 0);
    let chunk_mask = (1 << chunk_size) - 1;
    let mut chunks = Vec::new();
    
    for i in (0..width).step_by(chunk_size as usize) {
        let chunk = (number >> i) & chunk_mask;
        chunks.push(chunk as u64);
    }
    
    chunks
}

fn u8_array_to_u32_array(arr: &[u8; 32]) -> [u32; 8] {
    let mut result: [u32; 8] = [0; 8];

    for i in 0..8 {
        let sub_arr = &arr[i*4 .. (i+1)*4];
        result[i] = calculate_value2(sub_arr);
    }

    result
}


// fn u8_array_to_u32_array2(arr: &[u8; 8]) -> [u32; 2] {
//     let mut result: [u32; 2] = [0; 2];

//     for i in 0..2 {
//         result[i] = calculate_value2(&arr[i*4 .. (i+1)*4]);
//     }

//     result
// }


fn calculate_value(byte_array: &[u8; 32]) -> BigUint {
    let mut value: BigUint = BigUint::from(0u128);
    
    for &byte in byte_array.iter() {
        value = (value << 8) | BigUint::from(byte);
    }
    
    value
}

fn calculate_value2(byte_array: &[u8]) -> u32 {
    let mut value: u32 = 0u32;
    
    for &byte in byte_array.iter() {
        value = (value << 8) | u32::from(byte);
    }
    
    value
}

fn calculate_value3(byte_array: &[u8]) -> u32 {
    let mut value: u32 = 0u32;
    
    for &byte in byte_array.iter() {
        value = (value << 8) | u32::from(byte);
    }
    
    value
}

fn calc_value(array: [u32; 8]) -> BigUint {
    let mut value: BigUint = BigUint::from(0u32);
    for i in (0..8) {
        value <<= 32;
        value += BigUint::from(array[i]);
    }
    
    value
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    const MSG: &str = "helloworld🌍";
    




    
    let (sk1, pk1) = generate_keypair();
    #[cfg(not(feature = "x25519"))]
    let (sk1, pk1) = (&sk1.serialize(), &pk1.serialize());
    

    let (sk2, pk2) = generate_keypair();
    #[cfg(not(feature = "x25519"))]
    let (sk2, pk2) = (&sk2.serialize(), &pk2.serialize());

    //let ex1 = [254u8, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 200];
    let ex1 = [42, 106, 154, 98, 72, 133, 11, 18, 148, 49, 124, 163, 86, 169, 197, 242, 88, 88, 208, 68, 250, 147, 155, 212, 124, 88, 83, 65, 248, 191, 198, 160];
    
    print!("Ex1 value {:?}\n", calculate_value(&ex1));

    //let ex2 = [254u8, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 200];
    let ex2 = [170, 135, 166, 108, 65, 41, 7, 110, 97, 128, 15, 193, 132, 68, 157, 172, 127, 43, 197, 134, 247, 225, 146, 223, 13, 188, 59, 29, 27, 26, 178, 182];
    print!("Ex2 value {:?}\n", calculate_value(&ex2));


    let value_ex1 = BigUint::from_bytes_be(&ex1);

    print!("ex1 value: 1  {:?}\n", value_ex1.to_string());
    print!("ex1 value: 2 {:?}\n", calculate_value(&ex1).to_string());

    // Create a [u32; 8] array by compressing the [u8; 32] array
    let mut u32_array: [u32; 8] = [0; 8];
    for i in 0..8 {
        let start_index = i * 4;
        u32_array[i] = ((ex1[start_index] as u32) << 24)
            | ((ex1[start_index + 1] as u32) << 16)
            | ((ex1[start_index + 2] as u32) << 8)
            | (ex1[start_index + 3] as u32);
    }

    let compressed_ex1 = u8_array_to_u32_array(&ex1);
    print!("compressed_ex1 : 1 {:?}\n", compressed_ex1);
    print!("compressed_ex1 : 2 {:?}\n", u32_array);


    // Convert the [u32; 8] array to a BigUint

    let compressed_x1_value = calc_value(compressed_ex1);
    print!("ex1 value: 3 {:?}\n", compressed_x1_value.to_string());


    // print!("sk1 {:?}\n", sk1);
    // print!("sk2 {:?}\n", sk2);
    // print!("sk1 value {:?}\n", calculate_value(sk1));
    // print!("sk2 value {:?}\n", calculate_value(sk2));
    // print!("sk1 + sk2 {:?}\n", calculate_value(sk1) + calculate_value(sk2));


    print!("Sum(ex1 + ex2) {:?}\n", calculate_value(&ex1) + calculate_value(&ex2));


    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);
    let public_key = CompactPublicKey::new(&client_key);

    let start = Instant::now();

    set_server_key(server_keys);


    let aaa = FheUint32::try_encrypt(65535u16, &public_key).unwrap();
    let bbb = FheUint32::try_encrypt(65535u16, &public_key).unwrap();
    let ccc = &aaa + &bbb;
    let ddd: u32 = ccc.decrypt(&client_key);

    println!("DDDDDDDDDDDDDDDDDDDDDDDDD: {}", ddd);


    let mut c1 = Vec::new();

    let xxx = FheUint32::try_encrypt(0u32, &public_key).unwrap();
    let yyy = FheUint32::try_encrypt(0u32, &public_key).unwrap();
    c1.push(xxx);
    c1.remove(0);

    let compressed_sk1 = u8_array_to_u32_array(&ex1);
    for (i, chunk) in compressed_sk1.iter().enumerate() {
        println!("Chunk {}: {}", i, chunk);
        c1.push(FheUint32::try_encrypt(*chunk, &public_key).unwrap());
    }

    let mut c2 = Vec::new();
    c2.push(yyy);
    c2.remove(0);

    let compressed_sk2 = u8_array_to_u32_array(&ex2);
    for (i, chunk) in compressed_sk2.iter().enumerate() {
        println!("Chunk {}: {}", i, chunk);
        c2.push(FheUint32::try_encrypt(*chunk, &public_key).unwrap());
    }

    let start2 = Instant::now();

    let mut c = Vec::new();
    for i in 0..8 {
        c.push(&c1[i] + &c2[i]);
    }

    let duration2 = start2.elapsed();
    println!("Time elapsed for calculating C () is: {:?}", duration2);

  
    for element in c.iter() {
        let decrypted_ci : u64 = element.decrypt(&client_key);
        println!("Deeecrpted i {}", decrypted_ci);


    }

    let np1: u32 = c[7].decrypt(&client_key);
    println!("np1 = {}", np1);
    let np2: u32 = c[6].decrypt(&client_key);
    println!("np2 = {}", np2);
    let np3: u32 = c[5].decrypt(&client_key);
    println!("np3 = {}", np3);
    let np4: u32 = c[4].decrypt(&client_key);
    println!("np4 = {}", np4);
    let np5: u32 = c[3].decrypt(&client_key);
    println!("np5 = {}", np5);
    let np6: u32 = c[2].decrypt(&client_key);
    println!("np6 = {}", np6);
    let np7: u32 = c[1].decrypt(&client_key);
    println!("np7 = {}", np7);
    let np8: u32 = c[0].decrypt(&client_key);
    println!("np8 = {}", np8);

    // Combine the results to get the final sum
    let result = (BigUint::from(np1) << 224)
        | (BigUint::from(np2) << 192)
        | (BigUint::from(np3) << 160)
        | (BigUint::from(np4) << 128)
        | (BigUint::from(np5) << 96)
        | (BigUint::from(np6) << 64)
        | (BigUint::from(np7) << 32)
        | BigUint::from(np8);

    println!("Result: {:?}", result);

    // let compressed_x1_value = calc_value(c);
    // print!("ex1 value: 3 {:?}\n", compressed_x1_value.to_string());

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

 

    Ok(())
}