-- Add migration script here
CREATE TABLE notes(
  id VARCHAR(255) PRIMARY KEY,
  content TEXT NOT NULL
);