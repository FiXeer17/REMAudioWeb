CREATE TABLE presets(
    id SERIAL,
    label TEXT,
    relative_identifier INTEGER NOT NULL,
    socket_id INTEGER NOT NULL,
    FOREIGN KEY (socket_id) REFERENCES sockets(id) ON DELETE CASCADE
);

ALTER TABLE sockets
ADD COLUMN device TEXT NOT NULL;
