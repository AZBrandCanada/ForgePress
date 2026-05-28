# ForgePress: Core Architectural Blueprint & Design Specification

ForgePress is an open-source, highly modular content management system (CMS) built in Rust. It decouples content storage and administration from content delivery, bringing modern systems performance, type safety, and sandboxed extensibility to web publishing.

Instead of traditional, regex-heavy rendering loops and brittle database structures, ForgePress uses a decoupled **JSONB Block Architecture**, a sandboxed **WebAssembly (Wasm) and Rhai plugin engine**, and a high-performance **in-memory concurrently invalidated template engine**.

---

## 1. Core Architectural Concepts

To achieve the ease of use of WordPress without its performance and security drawbacks, ForgePress implements three core design patterns:

### A. The Decoupled JSONB Layout Strategy
Traditional CMS platforms rely on complex relational joins (EAV schemas/postmeta) or rigid table structures to handle layout builders. ForgePress keeps metadata relational (slugs, authors, status) but stores visual layouts and custom fields in a single PostgreSQL/SQLite binary JSON (`JSONB`) column called `content`.
* **Atomic Updates:** Saving a page requires only a single `UPDATE` query on one cell.
* **No Database Migrations:** When a developer builds a custom page-builder widget, they do not need to alter database tables. They simply save their data format into the JSON array.
* **High Performance:** Standard page loading requires only a single `SELECT` query, matching the page slug.

### B. Sandboxed Extensibility (Rhai & WebAssembly)
To prevent rogue plugin code from crashing the web server or accessing unauthorized files, ForgePress isolates dynamic code:
* **Lightweight Scripting (Rhai):** Safe, fast, sandboxed scripts that run inline using Rust-native bindings to intercept hooks and filters (e.g., `before_post_save`).
* **Binary Extensions (Wasm):** Complex plugins compile to WebAssembly components and run inside a sandboxed `wasmtime` runtime, utilizing a strict Guest/Host interface contract defined with WASI Preview 2 and WIT.

### C. Zero-JS Theme Rendering
Themes are written using standard HTML with Jinja-style placeholders (`MiniJinja`). The Rust template engine recursively processes the JSONB block tree, compiles the HTML fragments, and streams the finished page to the user at static-file speeds.

---

## 2. Project Directory Structure

ForgePress is organized as a multi-crate Rust workspace, separating the core server engine, the developer SDK, the command-line utility, and the frontend administration panel.

```text
forgepress/                                  # Workspace Root
├── Cargo.toml                               # Defines workspace members
├── .env.example                             # Template for DB configs, ports, and secret keys
├── migrations/                              # Database schemas (Handles SQLite & Postgres)
│   ├── postgres/                            # PG migration scripts (with JSONB indexes)
│   │   ├── 0001_create_users_table.sql
│   │   ├── 0002_create_pages_table.sql
│   │   └── 0003_create_taxonomies_table.sql
│   └── sqlite/                              # Mirror SQLite migration scripts
│
├── content/                                 # User-managed directory (Matches "wp-content")
│   ├── uploads/                             # Dynamic media assets directory
│   │   └── 2026/                            # Media parsed by Year/Month
│   │       └── 05/
│   │           ├── header-bg.jpg            # Original file
│   │           ├── header-bg-thumbnail.webp # Processed thumbnail
│   │           └── header-bg-large.webp     # Processed responsive large size
│   ├── themes/                              # Theme designs (HTML-only, no compiling needed)
│   │   └── default/
│   │       ├── theme.toml                   # Manifest (author, template configurations)
│   │       ├── templates/
│   │       │   ├── layouts/                 # Outer wrappers
│   │       │   │   ├── base.html            # Core HTML frame (CSS/JS injection points)
│   │       │   │   ├── header.html
│   │       │   │   └── footer.html
│   │       │   ├── single.html              # Template for individual posts/pages
│   │       │   ├── archive.html             # Template for lists (categories, tags, dates)
│   │       │   ├── 404.html                 # Fallback template
│   │       │   └── blocks/                  # Render engine components (JSONB block mapping)
│   │       │       ├── hero_section.html
│   │       │       └── rich_text.html
│   │       └── assets/                      # Static theme files (CSS, JS, WebFonts)
│   └── plugins/                             # Drop-in extensions
│       └── contact-forms/
│           ├── plugin.toml                  # Manifest (metadata, API settings, schema options)
│           ├── handler.rhai                 # Active Rhai scripting logic
│           └── assets/                      # Frontend script/styling for block rendering
│
├── forgepress-core/                         # The main Rust engine (Binary)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                          # Bootstraps Axum, connects DB, starts cron jobs
│       ├── app_state.rs                     # Handles DB pool, Cache, Template engine, Wasm engine
│       ├── config.rs                        # Safely parses env vars and system variables
│       ├── error.rs                         # Custom Enum to map DB, Auth, & Rendering errors
│       │
│       ├── auth/                            # Access Control Lists (ACL) & User Security
│       │   ├── mod.rs
│       │   ├── middleware.rs                # JWT / Session validations for /admin endpoints
│       │   ├── passwords.rs                 # Argon2 hashing utilities
│       │   └── roles.rs                     # Maps Roles: Admin, Editor, Contributor, Subscriber
│       │
│       ├── database/                        # Database interactions (SQLx implementation)
│       │   ├── mod.rs
│       │   ├── connection.rs                # Establishes PostgreSQL and SQLite connection pools
│       │   ├── pages.rs                     # Handles JSONB Page queries, CRUD, revisions
│       │   ├── users.rs                     # User profiles, auth lookups
│       │   ├── taxonomies.rs                # Maps posts to nested Categories, Tags, and Custom CPTs
│       │   └── options.rs                   # Key-value site settings (Equivalent to wp_options)
│       │
│       ├── domain/                          # Pure business logic structs (Type-safe schemas)
│       │   ├── mod.rs
│       │   ├── page.rs                      # Structures Page data and JSONB parsing types
│       │   ├── user.rs                      # User definitions
│       │   └── taxonomy.rs                  # Structural definitions of categories/tags
│       │
│       ├── routing/                         # Axum Request/Response Controllers
│       │   ├── mod.rs                       # Merges admin, public, and webhook routers
│       │   ├── admin_api/                   # REST API used by the admin dashboard / visual editor
│       │   │   ├── mod.rs
│       │   │   ├── auth.rs
│       │   │   ├── pages.rs                 # Handles PUT of visual editor JSONB payloads
│       │   │   └── media.rs
│       │   ├── public/                      # Front-end routing (Handles public traffic)
│       │   │   ├── mod.rs
│       │   │   ├── permalinks.rs            # Core dynamic routing rewrite decoder
│       │   │   └── renderer.rs              # Evaluates the JSONB and stitches templates
│       │   └── webhooks.rs                  # Public endpoints for third-party triggers
│       │
│       ├── media/                           # Asset optimizer (Eliminating manual scaling)
│       │   ├── mod.rs
│       │   ├── upload.rs                    # Validates, secures, and saves uploaded media
│       │   └── optimizer.rs                 # Automatically processes raw images to WebP/AVIF sizes
│       │
│       ├── cache/                           # Fragment caching & dependency invalidation
│       │   ├── mod.rs
│       │   ├── moka_cache.rs                # Thread-safe in-memory cache
│       │   └── invalidator.rs               # Tracks relationships to clear caches cleanly
│       │
│       ├── plugin_engine/                   # Extension loader
│       │   ├── mod.rs                       # Hook / Filter registry
│       │   ├── rhai_host.rs                 # Sandboxed Rhai scripts executor
│       │   └── wasm_host.rs                 # Sandboxed WebAssembly (wasmtime) runner
│       │
│       ├── template_engine/                 # Dynamic HTML Builder
│       │   ├── mod.rs                       # Combines blocks recursively into layouts
│       │   └── filters.rs                   # Custom template filters (e.g., date formats)
│       │
│       ├── jobs/                            # Background task scheduler (Like "wp-cron")
│       │   ├── mod.rs
│       │   └── scheduler.rs                 # Runs scheduled posts publish, sitemap gen, DB cleanup
│       │
│       └── i18n/                            # Localization
│           ├── mod.rs
│           └── loader.rs                    # Reads localization files, matches client HTTP locales
│
├── forgepress-plugin-sdk/                   # Library for external developers building Wasm plugins
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                           # Defines the exact interfaces plugins must export
│
├── forgepress-cli/                          # Command-line helper (Equivalent to wp-cli)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs                          # Commands: create-admin, install-theme, clear-cache
│
└── admin-dashboard/                         # Visual page builder & admin (React/Svelte SPA)
    ├── package.json
    ├── index.html
    └── src/                                 # Compiles to static assets embedded inside Rust binary