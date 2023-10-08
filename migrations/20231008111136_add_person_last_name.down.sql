-- Add down migration script here
ALTER TABLE people 
DROP COLUMN IF EXISTS last_name;