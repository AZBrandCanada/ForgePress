-- /migrations/sqlite/0001_create_users_table.sql
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'Subscriber',
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- Changed to TEXT
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP  -- Changed to TEXT
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);