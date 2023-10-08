-- Add up migration script here
ALTER TABLE people
ADD COLUMN last_name VARCHAR(40) NOT NULL;