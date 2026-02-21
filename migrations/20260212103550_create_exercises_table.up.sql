-- Add up migration script here
CREATE TABLE IF NOT EXISTS exercises (
    id SERIAL,
    name VARCHAR NOT NULL,
    weight INTEGER NOT NULL,
    reps INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    training_id INTEGER REFERENCES training(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS training_id_index ON exercises(training_id);
