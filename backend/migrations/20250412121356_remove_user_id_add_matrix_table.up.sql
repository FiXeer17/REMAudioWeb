ALTER TABLE channels
DROP COLUMN user_id;

CREATE TABLE sockets (
    id SERIAL PRIMARY KEY,
    socket_name TEXT,
    socket TEXT UNIQUE
);

ALTER TABLE channels
ADD COLUMN socket_id INTEGER,
ADD CONSTRAINT fk_socket
    FOREIGN KEY (socket_id) REFERENCES sockets(id) ON DELETE CASCADE;