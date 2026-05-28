#!/bin/bash
set -e

echo "=========================================================="
echo "      Forging ForgePress Workspace Directory Structure    "
echo "=========================================================="

# 1. Create root-level content folders
mkdir -p migrations/postgres
mkdir -p migrations/sqlite
mkdir -p content/uploads/2026/05
mkdir -p content/themes/default/templates/layouts
mkdir -p content/themes/default/templates/blocks
mkdir -p content/themes/default/assets
mkdir -p content/plugins/contact-forms/assets

# 2. Create forgepress-core subdirectories
mkdir -p forgepress-core/src/auth
mkdir -p forgepress-core/src/database
mkdir -p forgepress-core/src/domain
mkdir -p forgepress-core/src/routing/admin_api
mkdir -p forgepress-core/src/routing/public
mkdir -p forgepress-core/src/media
mkdir -p forgepress-core/src/cache
mkdir -p forgepress-core/src/plugin_engine
mkdir -p forgepress-core/src/template_engine
mkdir -p forgepress-core/src/jobs
mkdir -p forgepress-core/src/i18n

# 3. Create helper workspace crates and dashboard folders
mkdir -p forgepress-plugin-sdk/src
mkdir -p forgepress-cli/src
mkdir -p admin-dashboard/src

echo "Directories created successfully. Touching files..."

# 4. Touch Root Level files
touch Cargo.toml
touch .env.example

# 5. Touch migration placeholder files
touch migrations/postgres/0001_create_users_table.sql
touch migrations/postgres/0002_create_pages_table.sql
touch migrations/postgres/0003_create_taxonomies_table.sql
touch migrations/sqlite/.keep

# 6. Touch Theme files
touch content/themes/default/theme.toml
touch content/themes/default/templates/layouts/base.html
touch content/themes/default/templates/layouts/header.html
touch content/themes/default/templates/layouts/footer.html
touch content/themes/default/templates/single.html
touch content/themes/default/templates/archive.html
touch content/themes/default/templates/404.html
touch content/themes/default/templates/blocks/hero_section.html
touch content/themes/default/templates/blocks/rich_text.html
touch content/themes/default/assets/.keep

# 7. Touch Plugin files
touch content/plugins/contact-forms/plugin.toml
touch content/plugins/contact-forms/handler.rhai
touch content/plugins/contact-forms/assets/.keep

# 8. Touch Forgepress Core files
touch forgepress-core/Cargo.toml
touch forgepress-core/src/main.rs
touch forgepress-core/src/app_state.rs
touch forgepress-core/src/config.rs
touch forgepress-core/src/error.rs

# 9. Touch Auth files
touch forgepress-core/src/auth/mod.rs
touch forgepress-core/src/auth/middleware.rs
touch forgepress-core/src/auth/passwords.rs
touch forgepress-core/src/auth/roles.rs

# 10. Touch Database files
touch forgepress-core/src/database/mod.rs
touch forgepress-core/src/database/connection.rs
touch forgepress-core/src/database/pages.rs
touch forgepress-core/src/database/users.rs
touch forgepress-core/src/database/taxonomies.rs
touch forgepress-core/src/database/options.rs

# 11. Touch Domain entity files
touch forgepress-core/src/domain/mod.rs
touch forgepress-core/src/domain/page.rs
touch forgepress-core/src/domain/user.rs
touch forgepress-core/src/domain/taxonomy.rs

# 12. Touch Route files
touch forgepress-core/src/routing/mod.rs
touch forgepress-core/src/routing/admin_api/mod.rs
touch forgepress-core/src/routing/admin_api/auth.rs
touch forgepress-core/src/routing/admin_api/pages.rs
touch forgepress-core/src/routing/admin_api/media.rs

touch forgepress-core/src/routing/public/mod.rs
touch forgepress-core/src/routing/public/permalinks.rs
touch forgepress-core/src/routing/public/renderer.rs

touch forgepress-core/src/routing/webhooks.rs

# 13. Touch Optimizer and Media files
touch forgepress-core/src/media/mod.rs
touch forgepress-core/src/media/upload.rs
touch forgepress-core/src/media/optimizer.rs

# 14. Touch Cache files
touch forgepress-core/src/cache/mod.rs
touch forgepress-core/src/cache/moka_cache.rs
touch forgepress-core/src/cache/invalidator.rs

# 15. Touch Plugin Engine files
touch forgepress-core/src/plugin_engine/mod.rs
touch forgepress-core/src/plugin_engine/rhai_host.rs
touch forgepress-core/src/plugin_engine/wasm_host.rs

# 16. Touch Template Engine files
touch forgepress-core/src/template_engine/mod.rs
touch forgepress-core/src/template_engine/filters.rs

# 17. Touch Background Jobs files
touch forgepress-core/src/jobs/mod.rs
touch forgepress-core/src/jobs/scheduler.rs

# 18. Touch i18n translation files
touch forgepress-core/src/i18n/mod.rs
touch forgepress-core/src/i18n/loader.rs

# 19. Touch Other Workspace Crates files
touch forgepress-plugin-sdk/Cargo.toml
touch forgepress-plugin-sdk/src/lib.rs

touch forgepress-cli/Cargo.toml
touch forgepress-cli/src/main.rs

# 20. Touch Admin Dashboard files
touch admin-dashboard/package.json
touch admin-dashboard/index.html
touch admin-dashboard/src/.keep

echo "=========================================================="
echo "    ForgePress structure is ready. Happy coding!      "
echo "=========================================================="