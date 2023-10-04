-- Your SQL goes here
CREATE TABLE devices (
    id SERIAL PRIMARY KEY, 
    -- message_encryption_public_key bytea, 
    -- pke_public_key bytea, 
    -- fhe_public_key bytea, 
    -- fcm_code VARCHAR, 
    device_hash VARCHAR
);
