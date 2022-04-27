use std::path::PathBuf;

use realworld_db::Repository;
use sea_orm::Database;
pub struct Db(pub Repository);
use realworld_application::configuration::Settings;
use realworld_db::run_migrations;

impl Db {
    pub async fn create(url: &str) -> Db {
        run_migrations(url).await.expect("Failed to run migrations");
        let db = Database::connect(url).await.expect("Failed to connect");

        let repo = Repository::with_connection(db);
        Db(repo)
    }
}

pub async fn test_db(name: &str) -> Db {
    let settings = Settings::new(PathBuf::from("../../")).expect("Failed to load configuration");
    let db = Db::create(&settings.database.with_db(name).connection_string()).await;
    db
}
