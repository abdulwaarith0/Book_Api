-- Your SQL goes here


CREATE TABLE Books (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE
)