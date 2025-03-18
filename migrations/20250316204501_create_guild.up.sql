-- Add up migration script here
CREATE TABLE guilds (
  id UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  owner_id UUID NOT NULL,
  icon VARCHAR(255),
  description VARCHAR(255),
  banner VARCHAR(255)
);