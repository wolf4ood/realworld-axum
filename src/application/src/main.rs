use db::Repository;
use realworld_application::configuration::Settings;
use std::{net::SocketAddr, path::PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web2::get_app;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let settings = Settings::new(PathBuf::default()).expect("Failed to load configuration");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_error_handling_and_dependency_injection=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = Repository::create(settings.database.connection_string())
        .await
        .expect("Failed to create repository");

    let app = get_app(state);
    let address: SocketAddr = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    )
    .parse()
    .unwrap();
    tracing::info!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
