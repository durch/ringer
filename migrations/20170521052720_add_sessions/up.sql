CREATE TABLE SESSIONS (
    id SERIAL PRIMARY KEY,
    ext_id TEXT NOT NULL,
    valid_until TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX sessions_ext_id ON SESSIONS(ext_id);