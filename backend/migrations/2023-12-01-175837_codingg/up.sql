-- Your SQL goes here
CREATE TABLE users (
    username VARCHAR NOT NULL PRIMARY KEY,
    password VARCHAR NOT NULL
);

CREATE TABLE posts(
  id SERIAL PRIMARY KEY,
  author VARCHAR NOT NULL,
  location VARCHAR NOT NULL,
  content VARCHAR NOT NULL
)