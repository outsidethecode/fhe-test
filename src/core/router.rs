use rocket;

use crate::connection;
use crate::core;

pub fn create_routes() {
    rocket::ignite()
        .manage(connection::init_pool())
        .mount("/api",
               routes![
                    core::handler::all_devices,
                    core::handler::create_device,
                    core::handler::get_device,
                    ],
        ).launch();
}