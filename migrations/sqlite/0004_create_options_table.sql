-- /migrations/sqlite/0004_create_options_table.sql
CREATE TABLE options (
    option_key TEXT PRIMARY KEY,
    option_value TEXT NOT NULL
);