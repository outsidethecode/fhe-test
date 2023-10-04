use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

use crate::connection::DbConn;
use crate::core;
use crate::core::model::Device;
use crate::core::model::NewDevice;

#[get("/all_devices")]
pub fn all_devices(connection: DbConn) -> Result<Json<Vec<Device>>, Status> {
    core::repository::show_devices(&connection)
        .map(|device| Json(device))
        .map_err(|error| error_status(error))
}

#[post("/devices", format ="application/json", data = "<new_device>")]
pub fn create_device(new_device: Json<NewDevice>, connection: DbConn) ->  Result<status::Created<Json<Device>>, Status> {
    core::repository::create_device(new_device.into_inner(), &connection)
        .map(|device| device_created(device))
        .map_err(|error| error_status(error))

}

#[get("/<id>")]
pub fn get_device(id: i32, connection: DbConn) -> Result<Json<Device>, Status> {
    core::repository::get_device(id, &connection)
        .map(|device| Json(device))
        .map_err(|error| error_status(error))
}

fn device_created(device: Device) -> status::Created<Json<Device>> {
    status::Created(
        format!("{host}:{port}/device/{id}", host = host(), port = port(), id = device.id).to_string(),
        Some(Json(device)))
}

fn host() -> String {
    env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
    env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}