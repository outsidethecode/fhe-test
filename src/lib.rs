pub mod models;
pub mod schema;

use self::models::{Device, NewDevice};
use crate::schema::devices;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_device(
    connection: &mut PgConnection,
    message_encryption_public_key: &Vec<u8>,
    pke_public_key: &Vec<u8>,
    fcm_code: &String,
    device_hash: &String,
) -> Device {
    let new_device = NewDevice {
        message_encryption_public_key,
        pke_public_key,
        fcm_code,
        device_hash,
    };

    diesel::insert_into(devices::table)
        .values(&new_device)
        .returning(Device::as_returning())
        .get_result(connection)
        .expect("Error saving new device")
}

pub fn get_devices(
    connection: &mut PgConnection,
    device_hashes: Vec<String>
) -> Vec<Device> {

    use self::schema::devices::dsl::*;

    let results: Vec<_> = devices
        .filter(device_hash.eq_any(device_hashes))
        .limit(5)
        .select(Device::as_select())
        .load(connection)
        .expect("Error loading posts");
    
    results
}

