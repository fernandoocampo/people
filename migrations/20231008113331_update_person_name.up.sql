-- Add up migration script here
ALTER TABLE people 
RENAME COLUMN name TO first_name;