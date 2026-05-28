// /forgepress-core/src/jobs/scheduler.rs
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, error, debug};
use chrono::Utc;
use sqlx::Row;

use crate::app_state::AppState;
use crate::error::AppError;

async fn publish_scheduled_posts(state: &AppState) -> Result<(), AppError> {
    // Convert to ISO 8601 string to satisfy SQLx Any portable serialization rules
    let now_str = Utc::now().to_rfc3339();

    // 1. Fetch slugs using string comparison (works natively in Postgres and SQLite)
    let due_slugs: Vec<String> = sqlx::query(
        "SELECT slug FROM pages WHERE status = 'scheduled' AND published_at <= $1"
    )
    .bind(&now_str)
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|row| row.get::<String, _>("slug"))
    .collect();

    if due_slugs.is_empty() {
        return Ok(());
    }

    // 2. Perform the update query
    sqlx::query(
        "UPDATE pages SET status = 'published', updated_at = $1 \
         WHERE status = 'scheduled' AND published_at <= $1"
    )
    .bind(&now_str)
    .execute(&state.db)
    .await?;

    // 3. Purge caches
    for slug in due_slugs {
        state.cache.invalidate(&slug).await;
        info!("Scheduler published post: '/{}' and flushed cache.", slug);
    }

    Ok(())
}

async fn generate_sitemap(state: &AppState) -> Result<(), AppError> {
    debug!("Generating sitemap.xml...");

    let pages: Vec<String> = sqlx::query(
        "SELECT slug FROM pages WHERE status = 'published'"
    )
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|row| row.get::<String, _>("slug"))
    .collect();

    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    
    for slug in pages {
        let url_path = if slug == "index" { "".to_string() } else { slug };
        xml.push_str(&format!("  <url>\n    <loc>/{}</loc>\n  </url>\n", url_path));
    }
    
    xml.push_str("</urlset>");

    let sitemap_path = std::path::Path::new(&state.config.upload_dir).join("sitemap.xml");
    tokio::fs::write(&sitemap_path, xml).await?;
    
    debug!("Sitemap successfully generated at: {:?}", sitemap_path);
    Ok(())
}

async fn db_housekeeping(state: &AppState) -> Result<(), AppError> {
    debug!("Running database housekeeping routines...");
    
    let affected = sqlx::query(
        "DELETE FROM pages_taxonomies WHERE page_id NOT IN (SELECT id FROM pages)"
    )
    .execute(&state.db)
    .await?
    .rows_affected();

    if affected > 0 {
        info!("Database Housekeeping: Purged {} orphaned taxonomy associations.", affected);
    }

    Ok(())
}

async fn run_scheduled_jobs(state: &AppState) {
    if let Err(e) = publish_scheduled_posts(state).await {
        error!("Scheduler Job Failed (Publish Posts): {:?}", e);
    }
    
    if let Err(e) = generate_sitemap(state).await {
        error!("Scheduler Job Failed (Sitemap Gen): {:?}", e);
    }

    if let Err(e) = db_housekeeping(state).await {
        error!("Scheduler Job Failed (DB Housekeeping): {:?}", e);
    }
}

pub fn start_scheduler(state: AppState) {
    tokio::spawn(async move {
        let mut timer = interval(Duration::from_secs(60));
        timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!("ForgePress background task scheduler daemon started.");

        loop {
            timer.tick().await;
            run_scheduled_jobs(&state).await;
        }
    });
}