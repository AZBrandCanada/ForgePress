-- /migrations/sqlite/0002_create_pages_table.sql
CREATE TABLE pages (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    author_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    content TEXT NOT NULL DEFAULT '[]' CHECK(json_valid(content)),
    meta TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(meta)),
    published_at TEXT,                         -- Changed to TEXT
    created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- Changed to TEXT
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP  -- Changed to TEXT
);

CREATE INDEX idx_pages_slug ON pages(slug);
CREATE INDEX idx_pages_status ON pages(status);