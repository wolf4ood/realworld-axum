pub mod entity;
mod repository;
mod shims;

pub use repository::Repository;

use sqlx::migrate::MigrateDatabase;
use sqlx::PgPool;

pub async fn run_migrations(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let _ = sqlx::Any::drop_database(url).await;
    let _ = sqlx::Any::create_database(url).await;
    let pool = PgPool::connect(url).await.unwrap();
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool.close().await;
    Ok(())
}
