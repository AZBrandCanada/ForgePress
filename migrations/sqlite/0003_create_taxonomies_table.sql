-- /migrations/sqlite/0003_create_taxonomies_table.sql
CREATE TABLE taxonomies (
    id TEXT PRIMARY KEY, -- UUID stored as text
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    taxonomy_type TEXT NOT NULL, -- 'category', 'tag', or custom
    UNIQUE (slug, taxonomy_type)
);

CREATE TABLE pages_taxonomies (
    page_id TEXT REFERENCES pages(id) ON DELETE CASCADE,
    taxonomy_id TEXT REFERENCES taxonomies(id) ON DELETE CASCADE,
    PRIMARY KEY (page_id, taxonomy_id)
);

CREATE INDEX idx_pages_taxonomies_page ON pages_taxonomies(page_id);
CREATE INDEX idx_pages_taxonomies_tax ON pages_taxonomies(taxonomy_id);