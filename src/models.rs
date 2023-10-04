use diesel::prelude::*;
use crate::schema::devices;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::devices)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Device {
    pub message_encryption_public_key: Option<Vec<u8>>,
    pub pke_public_key: Option<Vec<u8>>,
    pub fcm_code: Option<String>,
    pub device_hash: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = devices)]
pub struct NewDevice<'a> {
    pub message_encryption_public_key: &'a Vec<u8>,
    pub pke_public_key: &'a Vec<u8>,
    pub fcm_code: &'a String,
    pub device_hash: &'a String,
}
