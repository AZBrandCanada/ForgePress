-- /migrations/sqlite/0002_create_pages_table.sql
CREATE TABLE pages (
    id TEXT PRIMARY KEY, -- UUID stored as text
    title TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    author_id TEXT REFERENCES users(id) ON DELETE SET NULL,
    content TEXT NOT NULL DEFAULT '[]' CHECK(json_valid(content)), -- Enforces valid JSON structure
    meta TEXT NOT NULL DEFAULT '{}' CHECK(json_valid(meta)),
    published_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_pages_slug ON pages(slug);
CREATE INDEX idx_pages_status ON pages(status);