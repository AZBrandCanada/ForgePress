-- /migrations/postgres/0003_create_taxonomies_table.sql
CREATE TABLE taxonomies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    taxonomy_type VARCHAR(100) NOT NULL, -- 'category', 'tag', or custom
    UNIQUE (slug, taxonomy_type)
);

CREATE TABLE pages_taxonomies (
    page_id UUID REFERENCES pages(id) ON DELETE CASCADE,
    taxonomy_id UUID REFERENCES taxonomies(id) ON DELETE CASCADE,
    PRIMARY KEY (page_id, taxonomy_id)
);

CREATE INDEX idx_pages_taxonomies_page ON pages_taxonomies(page_id);
CREATE INDEX idx_pages_taxonomies_tax ON pages_taxonomies(taxonomy_id);