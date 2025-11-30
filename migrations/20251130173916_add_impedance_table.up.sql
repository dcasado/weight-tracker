-- Add up migration script here
CREATE TABLE impedance (
    impedance_id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    measured_at TEXT NOT NULL,
    ohms REAL NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
)
