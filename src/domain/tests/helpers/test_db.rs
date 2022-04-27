use application::configuration::Postgres;
use realworld_db::run_migrations;
use realworld_db::Repository;
use sea_orm::Database;

pub struct Db(pub Repository);

impl Db {
    pub async fn create(pg: Postgres, name: &str) -> Db {
        let test = Postgres {
            db: name.to_string(),
            ..pg
        };

        run_migrations(&test.connection_string())
            .await
            .expect("Failed to run migrations");

        let db = Database::connect(test.connection_string())
            .await
            .expect("Failed to connect");

        let repo = Repository::with_connection(db);
        Db(repo)
    }
}
