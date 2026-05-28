-- /migrations/postgres/0002_create_pages_table.sql
CREATE TABLE pages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    author_id UUID REFERENCES users(id) ON DELETE SET NULL,
    content JSONB NOT NULL DEFAULT '[]'::jsonb, -- Stores the visual block layout tree
    meta JSONB NOT NULL DEFAULT '{}'::jsonb,    -- Stores unstructured SEO/metadata properties
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Fast routing search indexes
CREATE INDEX idx_pages_slug ON pages(slug);
CREATE INDEX idx_pages_status ON pages(status);

-- JSONB Path Index for fast searches inside the metadata schema
CREATE INDEX idx_pages_meta ON pages USING gin (meta);