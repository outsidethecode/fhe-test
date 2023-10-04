#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::core::model::Device;
use crate::core::model::NewDevice;

use crate::schema::devices;
use crate::schema::devices::dsl::*;

pub fn create_device(new_device: NewDevice, conn: &PgConnection) -> QueryResult<Device> {
    diesel::insert_into(devices::table)
        .values(&new_device)
        .get_result(conn)
}

pub fn show_devices(connection: &PgConnection) -> QueryResult<Vec<Device>>  {
    //devices.filter(published.eq(true))
    devices.limit(5)
        .load::<Device>(connection)
}

pub fn get_device(device_id: i32, connection: &PgConnection) -> QueryResult<Device> {
    devices::table.find(device_id).get_result::<Device>(connection)
}
