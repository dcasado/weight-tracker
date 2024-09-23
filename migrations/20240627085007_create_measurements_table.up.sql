-- Add up migration script here
CREATE TABLE measurements (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    date_time TEXT NOT NULL,
    weight REAL NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
)
