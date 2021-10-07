-- Your SQL goes here

CREATE TABLE IF NOT EXISTS persons (
       id             SERIAL PRIMARY KEY,
       first_name     VARCHAR NOT NULL,
       last_name      VARCHAR NOT NULL,
       active         BOOLEAN NOT NULL DEFAULT TRUE
);
