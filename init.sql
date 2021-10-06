CREATE DATABASE people;

CREATE TABLE IF NOT EXISTS person (
       id             SERIAL PRIMARY KEY,
       first_name     VARCHAR NOT NULL,
       last_name      VARCHAR NOT NULL
);
