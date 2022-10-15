-- Add migration script here
CREATE TABLE IF NOT EXISTS api_keys
(
    api_key varchar(100) PRIMARY KEY
);