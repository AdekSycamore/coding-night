-- Your SQL goes here
CREATE TABLE users (
  username VARCHAR NOT NULL PRIMARY KEY,
  password VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  region VARCHAR NOT NULL,
  firstname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL
);

CREATE TABLE posts(
  id SERIAL PRIMARY KEY,
  author VARCHAR NOT NULL,
  location VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  content VARCHAR NOT NULL,
  link VARCHAR NOT NULL,
  maplink VARCHAR NOT NULL
);