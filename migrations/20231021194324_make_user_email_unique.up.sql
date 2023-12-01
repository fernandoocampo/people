-- Add up migration script here
ALTER TABLE accounts
ADD CONSTRAINT email_uk UNIQUE (email);