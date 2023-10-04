#![feature(decl_macro)]
#[macro_use] extern crate rocket;

use rand::rngs::OsRng;
use rocket::data::{FromDataSimple, FromData};
use rocket::http::{ContentType, Status};
use secp256k1::{Secp256k1, ffi};
use ecies::{decrypt, encrypt, utils::generate_keypair};
use std::str::FromStr;
use std::time::{Duration, Instant};
use num_bigint::BigUint;
use serde::{Serialize, Deserialize};
use libsecp256k1::{self, SecretKey, PublicKey};
use rand::Rng;
use sunscreen::{
    fhe_program,
    types::{bfv::Signed, Cipher},
    Compiler, Error, FheRuntime,
};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    

    Ok(())
}