-- Add up migration script here
CREATE TABLE IF NOT EXISTS training (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    owner_id INTEGER REFERENCES users(telegram_id) ON DELETE CASCADE
);
