// /forgepress-core/src/jobs/scheduler.rs
use std::time::Duration;
use tokio::time::{interval, MissedTickBehavior};
use tracing::{info, error, debug};
use chrono::Utc;
use sqlx::Row;

use crate::app_state::AppState;
use crate::error::AppError;

/// Asynchronously publishes scheduled pages whose release times have arrived.
/// Purges their URLs from memory caches instantly.
async fn publish_scheduled_posts(state: &AppState) -> Result<(), AppError> {
    let now = Utc::now();

    // 1. Fetch slugs that are expired and due for publication
    let due_slugs: Vec<String> = sqlx::query(
        "SELECT slug FROM pages WHERE status = 'scheduled' AND published_at <= $1"
    )
    .bind(now)
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|row| row.get::<String, _>("slug"))
    .collect();

    if due_slugs.is_empty() {
        return Ok(());
    }

    // 2. Perform the update query to change status to 'published'
    sqlx::query(
        "UPDATE pages SET status = 'published', updated_at = $1 \
         WHERE status = 'scheduled' AND published_at <= $1"
    )
    .bind(now)
    .execute(&state.db)
    .await?;

    // 3. Purge caches immediately for all newly published pages
    for slug in due_slugs {
        state.cache.invalidate(&slug).await;
        info!("Scheduler published post: '/{}' and flushed cache.", slug);
    }

    Ok(())
}

/// Generates a standardized public sitemap.xml file.
async fn generate_sitemap(state: &AppState) -> Result<(), AppError> {
    debug!("Generating sitemap.xml...");

    // Fetch all published page slugs
    let pages: Vec<String> = sqlx::query(
        "SELECT slug FROM pages WHERE status = 'published'"
    )
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|row| row.get::<String, _>("slug"))
    .collect();

    // Build the XML payload
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    
    for slug in pages {
        // Fallback for root index path
        let url_path = if slug == "index" { "".to_string() } else { slug };
        xml.push_str(&format!("  <url>\n    <loc>/{}</loc>\n  </url>\n", url_path));
    }
    
    xml.push_str("</urlset>");

    // Save sitemap asynchronously to public upload directory
    let sitemap_path = std::path::Path::new(&state.config.upload_dir).join("sitemap.xml");
    tokio::fs::write(&sitemap_path, xml).await?;
    
    debug!("Sitemap successfully generated at: {:?}", sitemap_path);
    Ok(())
}

/// Cleans up orphaned taxonomies or database metadata.
async fn db_housekeeping(state: &AppState) -> Result<(), AppError> {
    debug!("Running database housekeeping routines...");
    
    // Delete orphaned relationships in the join table
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

/// Consolidates all background tasks. Wrapped in a safe try-catch wrapper
/// to ensure a single failed job can never crash the background scheduler.
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

/// Entry point to start the continuous background scheduler.
/// Spawns a non-blocking background task loop inside tokio's runtime.
pub fn start_scheduler(state: AppState) {
    tokio::spawn(async move {
        // Run background tasks every 60 seconds
        let mut timer = interval(Duration::from_secs(60));
        
        // EDGE CASE: If a task takes longer than 60 seconds or the CPU sleeps,
        // do not queue up or double-run missed ticks. Skip them!
        timer.set_missed_tick_behavior(MissedTickBehavior::Skip);

        info!("ForgePress background task scheduler daemon started.");

        loop {
            timer.tick().await;
            run_scheduled_jobs(&state).await;
        }
    });
}