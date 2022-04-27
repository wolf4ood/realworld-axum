#![allow(dead_code)]

pub mod generate;
pub mod test_db;
pub mod test_server;

use crate::helpers::generate::With;
use domain::repositories::Repository as RepositoryTrait;
use realworld_db::Repository;

use futures::future::join_all;
use futures::future::BoxFuture;
use futures::FutureExt;

use self::test_db::Db;

pub async fn create_users(db: &Db, num_users: i32) -> Vec<(domain::User, String)> {
    let users: Vec<BoxFuture<_>> = (0..num_users).map(|_| create_user(db).boxed()).collect();

    join_all(users).await
}

pub async fn create_user(db: &Db) -> (domain::User, String) {
    let (sign_up, clear_text_password) = generate::new_user();

    let user = db.0.sign_up(sign_up).await.expect("Failed to create user");

    (user, clear_text_password)
}

pub async fn create_articles(repo: &Db, users: Vec<domain::User>) -> Vec<domain::Article> {
    let articles = users
        .iter()
        .map(|user| create_article(repo, &user).boxed())
        .collect::<Vec<_>>();

    join_all(articles).await
}

pub async fn create_article(repo: &Db, user: &domain::User) -> domain::Article {
    let draft = generate::article_content();
    let author: domain::User = user.to_owned();

    repo.0.publish_article(draft, &author).await.unwrap()
}

pub async fn create_users2(repo: &Repository, num_users: i32) -> Vec<(domain::User, String)> {
    let users: Vec<BoxFuture<_>> = (0..num_users).map(|_| create_user2(repo).boxed()).collect();

    join_all(users).await
}

pub async fn create_user2(repo: &Repository) -> (domain::User, String) {
    let (new_user, password) = generate::new_user();
    let new_user = repo
        .sign_up(new_user)
        .await
        .expect("Failed to create user")
        .into();
    (new_user, password)
}

pub async fn create_article2(repo: &Repository, author: With<&domain::User>) -> domain::Article {
    let author = match author {
        With::Random => create_user2(repo).await.0,
        With::Value(user) => user.to_owned(),
    };
    let draft = generate::article_content();
    author.publish(draft, repo).await.unwrap()
}
