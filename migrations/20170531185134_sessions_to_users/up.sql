ALTER TABLE SESSIONS ADD COLUMN user_id INTEGER NOT NULL REFERENCES users(id);