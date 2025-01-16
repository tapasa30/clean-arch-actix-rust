-- Add up migration script here
CREATE TABLE demo_models (
    id UUID PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE
);
