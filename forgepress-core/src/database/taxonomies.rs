// /forgepress-core/src/database/taxonomies.rs
use sqlx::AnyPool;
use crate::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Taxonomy {
    pub id: String, 
    pub name: String,
    pub slug: String,
    pub taxonomy_type: String,
}

pub async fn create_taxonomy(
    pool: &AnyPool,
    name: &str,
    slug: &str,
    taxonomy_type: &str,
) -> Result<Taxonomy, AppError> {
    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO taxonomies (id, name, slug, taxonomy_type) VALUES (CAST($1 AS uuid), $2, $3, $4)"
    )
    .bind(&id)
    .bind(name)
    .bind(slug)
    .bind(taxonomy_type)
    .execute(pool)
    .await?;

    Ok(Taxonomy {
        id,
        name: name.to_string(),
        slug: slug.to_string(),
        taxonomy_type: taxonomy_type.to_string(),
    })
}

pub async fn link_page_to_taxonomy(pool: &AnyPool, page_id: &str, taxonomy_id: &str) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO pages_taxonomies (page_id, taxonomy_id) VALUES (CAST($1 AS uuid), CAST($2 AS uuid)) \
         ON CONFLICT DO NOTHING"
    )
    .bind(page_id)
    .bind(taxonomy_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_taxonomies_for_page(pool: &AnyPool, page_id: &str) -> Result<Vec<Taxonomy>, AppError> {
    let taxonomies = sqlx::query_as::<_, Taxonomy>(
        "SELECT CAST(t.id AS VARCHAR) AS id, t.name, t.slug, t.taxonomy_type FROM taxonomies t \
         JOIN pages_taxonomies pt ON pt.taxonomy_id = t.id \
         WHERE pt.page_id = CAST($1 AS uuid)"
    )
    .bind(page_id)
    .fetch_all(pool)
    .await?;
    Ok(taxonomies)
}