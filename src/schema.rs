// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Int4,
        device_hash -> Varchar,
    }
}
