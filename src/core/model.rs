#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::devices;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "devices"]
pub struct Device {
    pub id: i32,
    // pub message_encryption_public_key: Option<Vec<u8>>,
    // pub pke_public_key: Option<Vec<u8>>,
    // pub fcm_code: String,
    pub device_hash: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="devices"]
pub struct NewDevice {
    // pub message_encryption_public_key: Vec<u8>,
    // pub pke_public_key: Vec<u8>,
    // pub fcm_code: String,
    pub device_hash: String,
}

