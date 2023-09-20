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


fn main() -> Result<(), Box<dyn std::error::Error>> {


    const MSG: &str = "helloworld🌍";
    
    
    let (sk1, pk1) = generate_keypair();
    #[cfg(not(feature = "x25519"))]
    let (sk1, pk1) = (&sk1.serialize(), &pk1.serialize());
    

    let (sk2, pk2) = generate_keypair();
    #[cfg(not(feature = "x25519"))]
    let (sk2, pk2) = (&sk2.serialize(), &pk2.serialize());

    let ex1 = [200u8, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200];
    print!("Ex1 value {:?}\n", calculate_value(&ex1));

    let ex2 = [200u8, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200, 200];
    print!("Ex1 value {:?}\n", calculate_value(&ex2));


    
    print!("sk1 {:?}\n", sk1);
    print!("sk2 {:?}\n", sk2);
    print!("sk1 value {:?}\n", calculate_value(sk1));
    print!("sk2 value {:?}\n", calculate_value(sk2));
    print!("sk1 + sk2 {:?}\n", calculate_value(sk1) + calculate_value(sk2));
    let compressed_ex2 = u8_array_to_u32_array(&ex2);

    print!("compressed_Ex2 {:?}\n", compressed_ex2);


    print!("Sum(ex1 + ex2) {:?}\n", calculate_value(&ex1) + calculate_value(&ex2));
    // let pkz1 = pk1 + pk2;
   
    // let msg = MSG.as_bytes();
    // assert_eq!(
    //     msg,
    //     decrypt(sk, &encrypt(pk, msg).unwrap()).unwrap().as_slice()
    // );


    // Basic configuration to use homomorphic integers
    let config = ConfigBuilder::all_disabled()
        .enable_default_integers()
        .build();

    // Key generation
    let (client_key, server_keys) = generate_keys(config);
    let public_key = CompactPublicKey::new(&client_key);

    let start = Instant::now();

    set_server_key(server_keys);

    let mut c1 = Vec::new();

    let xxx = FheUint32::try_encrypt(0u32, &public_key).unwrap();
    let yyy = FheUint32::try_encrypt(0u32, &public_key).unwrap();
    c1.push(xxx);
    c1.remove(0);

    let compressed_sk1 = u8_array_to_u32_array(&ex1);
    for (i, chunk) in compressed_sk1.iter().enumerate() {
        println!("Chunk {}: {}", i, chunk);
        c1.push(FheUint32::try_encrypt(*chunk, &public_key).unwrap());

        // let shift : u32 = (i*32).try_into().unwrap();
        // let encrypted_shift = FheUint32::try_encrypt(shift, &public_key).unwrap();
        // encrypted_sk1 = encrypted_sk1 + &a << shift;
    }

    let mut c2 = Vec::new();
    c2.push(yyy);
    c2.remove(0);

    let compressed_sk2 = u8_array_to_u32_array(&ex2);
    for (i, chunk) in compressed_sk2.iter().enumerate() {
        println!("Chunk {}: {}", i, chunk);
        c2.push(FheUint32::try_encrypt(*chunk, &public_key).unwrap());
        // let shift : u32 = (i*32).try_into().unwrap();
        // let encrypted_shift = FheUint32::try_encrypt(shift, &public_key).unwrap();
        // encrypted_sk1 = encrypted_sk1 + &a << shift;
    }


    let start2 = Instant::now();


    let mut c = Vec::new();
    for i in 0..8 {
        c.push(&c1[i] + &c2[i]);
    }

    let duration2 = start2.elapsed();
    println!("Time elapsed for calculating C () is: {:?}", duration2);


    

    // let mut skz1 : BigUint = BigUint::from(0u64);
    // for i in 0..8 {
    //     let decrypted_ci : u32 = c[i].decrypt(&client_key);
    //     let base : u128 = 256;
    //     println!("Decccc {}", decrypted_ci );
    //     println!("poe {}", base.pow(i as u32) as u128);
    //     let decrypted_ci_u128: u128 = decrypted_ci.into();
    //     skz1 = skz1 + calculate_value(decrypted_ci);
    // }


    let mut value: BigUint = BigUint::from(0u64);
    
    for element in c.iter() {
        let decrypted_ci : u128 = element.decrypt(&client_key);
        println!("Deeecrpted i {}", decrypted_ci);
        value = (value << 32) | BigUint::from(decrypted_ci);
    }


    println!("{}", value);



    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);


    // let start = Instant::now();


    // let dec_clear_a: u8 = a.decrypt(&client_key);

    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);



    // // Basic configuration to use homomorphic integers
    // let config = ConfigBuilder::all_disabled()
    //     .enable_default_integers()
    //     .build();

    // // Key generation
    // let (client_key, server_keys) = generate_keys(config);

    // // let clear_a = 
    // // let clear_b = 5u32;
    // // let clear_c = 7u8;

    
    // let clear_a = 12u32;
    // let clear_b = 10u32;

  
    // let public_key = CompactPublicKey::new(&client_key);

    // let a = FheUint32::try_encrypt(clear_a, &public_key).unwrap();
    // let b = FheUint32::try_encrypt(clear_b, &public_key).unwrap();



    // // Server side
    // set_server_key(server_keys);
    // let encrypted_res_mul = &a * &b;

    // let dec_clear_a: u8 = a.decrypt(&client_key);
    // let dec_clear_b: u8 = b.decrypt(&client_key);
    // let dec_clear: u8 = encrypted_res_mul.decrypt(&client_key);

    // print!("{}", dec_clear);


    

    Ok(())
}