use rsa::signature::digest::typenum::Unsigned;
use sunscreen::Ciphertext;
use tfhe::{prelude::*, FheUint16};
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint32, CompactPublicKey};
extern crate secp256k1;
extern crate hex;
use ecies::{decrypt, encrypt, utils::generate_keypair};
use std::str::FromStr;
use std::time::{Duration, Instant};
use num_bigint::BigUint;


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

fn calc_slice_value(byte_array: &[u8]) -> u32 {
    let mut value: u32 = 0u32;
    
    for &byte in byte_array.iter() {
        value = (value << 8) | u32::from(byte);
    }
    
    value
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

   
    let app = Compiler::new()
    .fhe_program(simple_add)
    .compile()?;

    let runtime = FheRuntime::new(app.params())?;

    let (public_key, private_key) = runtime.generate_keys()?;


    let a = runtime.encrypt(Signed::from(2147483647), &public_key)?;
    // let b = runtime.encrypt(Signed::from(2147483647), &public_key)?;

    // let start = Instant::now();

    // let results = runtime.run(app.get_fhe_program(simple_add).unwrap(), vec![a, b], &public_key)?;
    // let duration = start.elapsed();
    // println!("Time elapsed in expensive_function() is: {:?}", duration);

    // let c: Signed = runtime.decrypt(&results[0], &private_key)?;

    // println!("{}", c);
    // Ok(())




    let (sk1, pk1) = generate_keypair();
    #[cfg(not(feature = "x25519"))]
    let (sk1, pk1) = (&sk1.serialize(), &pk1.serialize());
    

    let (sk2, pk2) = generate_keypair();
    #[cfg(not(feature = "x25519"))]
    let (sk2, pk2) = (&sk2.serialize(), &pk2.serialize());

    print!("sk1 {:?}\n", sk1);
    print!("sk2 {:?}\n", sk2);
    print!("sk1 value {:?}\n", calculate_value(sk1));
    print!("sk2 value {:?}\n", calculate_value(sk2));
    print!("sk1 + sk2 {:?}\n", calculate_value(sk1) + calculate_value(sk2));

    let start = Instant::now();

    let mut c1 = Vec::new();
    let compressed_sk1 = u8_32_array_to_u32_8_array(&sk1);
    for (i, chunk) in compressed_sk1.iter().enumerate() {
        let i64_value : i64 = *chunk as i64;
        c1.push(runtime.encrypt(Signed::from(i64_value), &public_key)?);
    }

    let mut c2 = Vec::new();
    let compressed_sk2 = u8_32_array_to_u32_8_array(&sk2);
    for (i, chunk) in compressed_sk2.iter().enumerate() {
        let i64_value : i64 = *chunk as i64;
        c2.push(runtime.encrypt(Signed::from(i64_value), &public_key)?);
    }

    let mut c = Vec::new();
    for i in 0..8 {
        let a = c1[i].clone();
        let b = c2[i].clone();
        c.push(runtime.run(app.get_fhe_program(simple_add).unwrap(), vec![a, b], &public_key)?);
    }
  
    let mut value: BigUint = BigUint::from(0u128);
    for element in c.iter() {
        let decrypted_ci : Signed = runtime.decrypt(&element[0], &private_key)?;
        value <<= 32;
        value += BigUint::from_str(&decrypted_ci.to_string()).unwrap();
    }

    println!("SKz1: {:?}", value);

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

 

    Ok(())
}