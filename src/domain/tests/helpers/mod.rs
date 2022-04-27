#![allow(dead_code)]

pub mod generate;
pub mod test_db;

use std::path::PathBuf;

use crate::helpers::generate::With;
use application::configuration::Settings;
use futures::future::join_all;
use futures::future::BoxFuture;
use futures::FutureExt;
use realworld_domain::repositories::Repository as RepositoryTrait;
use realworld_domain::Article;
use realworld_domain::User;

use self::test_db::Db;

pub async fn test_db(name: &str) -> Db {
    let settings = Settings::new(PathBuf::from("../../")).expect("Failed to load configuration");
    Db::create(settings.database, name).await
}
pub async fn create_users(db: &Db, num_users: i32) -> Vec<(User, String)> {
    let users: Vec<BoxFuture<_>> = (0..num_users).map(|_| create_user(db).boxed()).collect();

    join_all(users).await
}

pub async fn create_user(db: &Db) -> (realworld_domain::User, String) {
    let (sign_up, clear_text_password) = generate::new_user();

    let user = db.0.sign_up(sign_up).await.expect("Failed to create user");

    (user, clear_text_password)
}

pub async fn create_articles(repo: &Db, users: Vec<User>) -> Vec<Article> {
    let articles = users
        .iter()
        .map(|user| create_article(repo, &user).boxed())
        .collect::<Vec<_>>();

    join_all(articles).await
}

pub async fn create_article(repo: &Db, user: &User) -> Article {
    let draft = generate::article_content();
    let author: User = user.to_owned();

    repo.0.publish_article(draft, &author).await.unwrap()
}

pub async fn create_article2(
    repo: &Db,
    author: With<&realworld_domain::User>,
) -> realworld_domain::Article {
    let author = match author {
        With::Random => create_user(repo).await.0,
        With::Value(user) => user.to_owned(),
    };
    let draft = generate::article_content();
    author.publish(draft, &repo.0).await.unwrap()
}
