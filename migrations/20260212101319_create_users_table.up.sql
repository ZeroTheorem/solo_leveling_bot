-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    telegram_id BIGINT NOT NULL PRIMARY KEY,
    lvl BIGINT NOT NULL,
    exp BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
