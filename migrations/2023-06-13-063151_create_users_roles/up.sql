-- Your SQL goes here
CREATE TABLE users_roles (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id integer NOT NULL REFERENCES users(id),
  role_id integer NOT NULL REFERENCES roles(id)
)