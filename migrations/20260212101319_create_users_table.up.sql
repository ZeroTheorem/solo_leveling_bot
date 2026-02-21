-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    telegram_id BIGINT NOT NULL PRIMARY KEY,
    lvl INTEGER NOT NULL,
    exp INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
