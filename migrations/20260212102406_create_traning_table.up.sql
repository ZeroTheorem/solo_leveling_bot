-- Add up migration script here
CREATE TABLE IF NOT EXISTS training (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    owner_id BIGINT REFERENCES users(telegram_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS owner_id_index ON training(owner_id);
