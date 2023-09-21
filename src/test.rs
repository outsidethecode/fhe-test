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

fn main() {
    let num1 = 1234567890;
    let num2 = 9876543210;

    let num = [8, 12, 5, 13];

    let mut value: BigUint = BigUint::from(0u128);
    
    let mut carry1: BigUint = BigUint::from(0u128);
    let mut carry: BigUint = BigUint::from(0u128);
    let mut result: BigUint = BigUint::from(0u128);
    let mut multiplier: BigUint = BigUint::from(1u128);

    for element in c.iter() {
        println!("Deeecrpted i {}", element);

        let remainder = element.clone() % BigUint::from(10u128);
        value = value + (remainder  + carry1.clone())* multiplier.clone();
        carry1 = element.clone() / BigUint::from(10u128);

        println!("Value i {}", value);
        println!("Carry i {}", carry1);
        println!("-----------");

        multiplier *= BigUint::from(10u128);

    }

    if carry1 > BigUint::from(0u128) {
        value = value + (carry1.clone())* multiplier.clone();
    }
    

    // let mut value: BigUint = BigUint::from(0u128);
    
    // let mut carry1: BigUint = BigUint::from(0u128);
    // let mut carry: BigUint = BigUint::from(0u128);
    // let mut result: BigUint = BigUint::from(0u128);
    // let mut multiplier: BigUint = BigUint::from(1u128);

    // let arr = [BigUint::from(12u32), BigUint::from(16u32), BigUint::from(14u32), BigUint::from(14u32)];

    // for element in arr.iter() {
    //     println!("Deeecrpted i {}", element);

    //     let remainder = element.clone() % BigUint::from(10u128);
    //     value = value + (remainder  + carry1.clone())* multiplier.clone();
    //     carry1 = element.clone() / BigUint::from(10u128);

    //     println!("Value i {}", value);
    //     println!("Carry i {}", carry1);
    //     println!("-----------");

    //     multiplier *= BigUint::from(10u128);

    // }

    // if carry1 > BigUint::from(0u128) {
    //     value = value + (carry1.clone())* multiplier.clone();
    // }

    // println!("Value {}", value);



    // let arr = [BigUint::from(70000u32), BigUint::from(6000u32)];

    // for element in arr.iter() {
    //     println!("Deeecrpted i {}", element);

    //     let remainder = element.clone() % BigUint::from(65536u128);
    //     value = value + (remainder  + carry1.clone())* multiplier.clone();
    //     carry1 = element.clone() / BigUint::from(65536u128);

    //     println!("Value i {}", value);
    //     println!("Carry i {}", carry1);
    //     println!("-----------");

    //     multiplier *= BigUint::from(65536u128);

    // }

    // if carry1 > BigUint::from(0u128) {
    //     value = value + (carry1.clone())* multiplier.clone();
    // }

    // println!("Value {}", value);


    let mut value: BigUint = BigUint::from(0u128);
    
    let mut carry1: BigUint = BigUint::from(0u128);
    let mut carry: BigUint = BigUint::from(0u128);
    let mut result: BigUint = BigUint::from(0u128);
    let mut multiplier: BigUint = BigUint::from(1u128);

    for element in c.iter() {
        let decrypted_ci : u32 = element.decrypt(&client_key);
        println!("Deeecrpted i {}", decrypted_ci);

        // value = (value << 16) | BigUint::from(decrypted_ci) + carry1.clone();
        let remainder = decrypted_ci.clone() % BigUint::from(65536u128);
        value = value + (remainder  + carry1.clone())* multiplier.clone();
        carry1 = decrypted_ci.clone() / BigUint::from(65536u128);


        // // if decrypted_ci > 65535u32 {
            

        // //     carry1 = BigUint::from(1u128);
        // // } else {
        // //     value = value + (decrypted_ci + carry1.clone())* multiplier.clone();

        // //     carry1 = BigUint::from(0u128);
        // // } 


        // // println!("Vallllue i {}", value);

        // let sum = BigUint::from(decrypted_ci) + carry;
        // carry = sum.clone() / BigUint::from(65535u128);
        // println!("Carry i {}", carry);

        // let remainder = sum.clone() % BigUint::from(65535u128);
        // result += remainder * multiplier.clone();

        println!("Value i {}", value);
        println!("Carry i {}", carry1);
        println!("-----------");

        multiplier *= BigUint::from(65536u128);

    }

    if carry1 > BigUint::from(0u128) {
        value = value + (carry1.clone())* multiplier.clone();
    }
    


    println!("Value {}", value);
    println!("Result {}", result);


    println!("Value {}", value);

}

fn add_unsigned_numbers(num1: u64, num2: u64) -> u64 {
    let mut num1_value = num1;
    let mut num2_value = num2;

    let mut carry = 0;
    let mut result = 0;
    let mut multiplier = 1;

    while num1_value > 0 || num2_value > 0 || carry > 0 {
        let digit1 = num1_value % 10;
        let digit2 = num2_value % 10;

        let sum = digit1 + digit2 + carry;
        carry = sum / 10;
        let remainder = sum % 10;

        result += remainder * multiplier;
        multiplier *= 10;

        num1_value /= 10;
        num2_value /= 10;
    }

    result
}