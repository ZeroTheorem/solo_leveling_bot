-- Add up migration script here
CREATE TABLE IF NOT EXISTS exercises (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    weight INTEGER NOT NULL,
    reps INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    training_id BIGINT REFERENCES training(id) ON DELETE CASCADE,
    owner_id BIGINT REFERENCES users(telegram_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS training_id_index ON exercises(training_id);
CREATE INDEX IF NOT EXISTS owner_id_index ON users(telegram_id);
