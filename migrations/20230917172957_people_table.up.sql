-- Add up migration script here
CREATE TABLE IF NOT EXISTS people (
    ID VARCHAR(36) PRIMARY KEY,
    NAME VARCHAR(40) NOT NULL,
    CREATED_ON TIMESTAMP NOT NULL DEFAULT NOW()
);