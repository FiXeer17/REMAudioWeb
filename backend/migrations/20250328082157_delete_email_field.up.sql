ALTER TABLE users
DROP COLUMN email;
ALTER TABLE users 
ADD CONSTRAINT unique_username UNIQUE (username);
