CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  task VARCHAR NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE users (
    username VARCHAR NOT NULL PRIMARY KEY,
    password VARCHAR NOT NULL
)
