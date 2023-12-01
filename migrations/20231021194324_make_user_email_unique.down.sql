-- Add down migration script here
ALTER TABLE accounts
ADD DROP CONSTRAINT email_uk;