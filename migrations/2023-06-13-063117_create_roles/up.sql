-- Your SQL goes here
CREATE TABLE roles (
  id SERIAL NOT NULL PRIMARY KEY,
  code varchar(64) NOT NULL UNIQUE,
  name varchar(128) NOT NULL,
  created_at TIMESTAMP DEFAULT NOW() NOT NULL
)