// /forgepress-cli/src/main.rs
use clap::{Parser, Subcommand};
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};

#[derive(Parser)]
#[command(name = "forgepress-cli")]
#[command(about = "Command Line Interface (CLI) helper for ForgePress", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Registers a new administrative user directly into the database.
    CreateAdmin {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    /// Scaffolds a structured, ready-to-edit template theme directory.
    InstallTheme {
        #[arg(short, long)]
        name: String,
    },
    /// Sends a signed request to invalidate server memory cache dynamically.
    ClearCache {
        /// Targeted page slug to invalidate, or "all" to purge completely
        #[arg(short, long, default_value = "all")]
        target: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _ = dotenvy::dotenv(); // Load .env configuration
    let cli = Cli::parse();

    match cli.command {
        Commands::CreateAdmin { username, email, password } => {
            println!("Connecting to database to register user: {}...", username);
            
            let db_url = std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://content/forgepress.db".to_string());

            install_default_drivers();
            let pool = AnyPoolOptions::new()
                .max_connections(2)
                .connect(&db_url)
                .await?;

            // 1. Hash password securely using Argon2id
            let salt = SaltString::generate(&mut OsRng);
            let argon2_hash = Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .map_err(|e| anyhow::anyhow!("Argon2 Hashing Error: {}", e))?
                .to_string();

            let id = Uuid::new_v4().to_string();
            let now = Utc::now();

            // 2. Insert user record with Administrator role
            sqlx::query(
                "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at) \
                 VALUES ($1, $2, $3, $4, 'Administrator', $5, $6)"
            )
            .bind(id)
            .bind(username.clone())
            .bind(email)
            .bind(argon2_hash)
            .bind(now)
            .bind(now)
            .execute(&pool)
            .await?;

            println!("Success! Administrator '{}' created successfully.", username);
        }

        Commands::InstallTheme { name } => {
            println!("Scaffolding theme folder: '{}'...", name);
            
            let content_dir = std::env::var("CONTENT_DIR").unwrap_or_else(|_| "./content".to_string());
            let theme_dir = Path::new(&content_dir).join("themes").join(&name);

            if theme_dir.exists() {
                println!("Error: Theme directory '{:?}' already exists.", theme_dir);
                std::process::exit(1);
            }

            // Create structure paths
            std::fs::create_dir_all(theme_dir.join("templates/layouts"))?;
            std::fs::create_dir_all(theme_dir.join("templates/blocks"))?;
            std::fs::create_dir_all(theme_dir.join("assets"))?;

            // Write theme manifest (theme.toml)
            let theme_toml_path = theme_dir.join("theme.toml");
            let theme_manifest = format!(
                "name = \"{}\"\nversion = \"1.0.0\"\nauthor = \"Admin\"\ncompatible_builders = [\"default-visual-builder\"]\n",
                name
            );
            std::fs::write(&theme_toml_path, theme_manifest)?;

            // Write template entry point placeholder
            let base_html_path = theme_dir.join("templates/layouts/base.html");
            std::fs::write(&base_html_path, "<!-- Scaffolded base wrapper layout -->\n")?;

            println!("Success! Scaffolding completed. Theme template folder ready at: {:?}", theme_dir);
        }

        Commands::ClearCache { target } => {
            let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
            let secret = std::env::var("SECRET_KEY")
                .unwrap_or_else(|_| "generate_a_random_64_character_string_for_production_use".to_string());

            println!("Sending secure cache purge webhook to active server on port {}...", port);

            let client = reqwest::Client::new();
            let url = format!("http://localhost:{}/api/webhooks/cache-purge", port);

            // Construct payload utilizing our private SECRET_KEY signature
            let payload = serde_json::json!({
                "secret": secret,
                "target": target
            });

            let response = client.post(&url)
                .json(&payload)
                .send()
                .await?;

            if response.status().is_success() {
                println!("Success! Memory caches matching target '{}' invalidated successfully.", target);
            } else {
                println!("Error: Failed to invalidate cache. Server responded with: {:?}", response.status());
            }
        }
    }

    Ok(())
}