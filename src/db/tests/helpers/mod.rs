#![allow(dead_code)]

use realworld_domain::{Article, User};

use realworld_tests::db::Db;
pub mod generate;
use futures::{
    future::{join_all, BoxFuture},
    FutureExt,
};
use realworld_domain::repositories::Repository;

pub async fn create_users(db: &Db, num_users: i32) -> Vec<(User, String)> {
    let users: Vec<BoxFuture<_>> = (0..num_users).map(|_| create_user(db).boxed()).collect();

    join_all(users).await
}

pub async fn create_user(db: &Db) -> (User, String) {
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
