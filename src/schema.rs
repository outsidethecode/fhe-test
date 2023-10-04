// @generated automatically by Diesel CLI.

diesel::table! {
    devices (id) {
        id -> Int4,
        message_encryption_public_key -> Nullable<Bytea>,
        pke_public_key -> Nullable<Bytea>,
        fhe_public_key -> Nullable<Bytea>,
        fcm_code -> Nullable<Varchar>,
        device_hash -> Nullable<Varchar>,
    }
}
