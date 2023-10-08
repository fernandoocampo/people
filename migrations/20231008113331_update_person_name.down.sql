-- Add down migration script here
ALTER TABLE people 
RENAME COLUMN first_name TO name;