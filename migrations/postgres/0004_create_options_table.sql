-- /migrations/postgres/0004_create_options_table.sql
CREATE TABLE options (
    option_key VARCHAR(255) PRIMARY KEY,
    option_value TEXT NOT NULL -- Stored as serialized JSON strings
);