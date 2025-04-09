CREATE TABLE channels(
    id SERIAL,
    channel_name TEXT,
    visible BOOLEAN,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);