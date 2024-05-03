CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS todos (
  id UUID PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  status SMALLINT NOT NULL DEFAULT 0,
  finished_at TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS todos_id_idx ON todos (id);

INSERT INTO todos (id, title, description, status, finished_at, created_at)
VALUES (uuid_generate_v4(), 'First TODO', 'This is the first todo', 0, NULL, CURRENT_TIMESTAMP);