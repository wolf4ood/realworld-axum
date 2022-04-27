use clap::Parser;
use realworld_application::configuration::Settings;
use sqlx_cli::{run, Opt};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new(PathBuf::default()).expect("Failed to load configuration");

    std::env::set_var("DATABASE_URL", settings.database.connection_string());
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    run(Opt::parse()).await?;

    Ok(())
}
