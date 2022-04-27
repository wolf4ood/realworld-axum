use crate::helpers::{create_articles, create_users};
use realworld_domain::{repositories::Repository, User};
use realworld_tests::db::test_db;
use sea_orm::EntityTrait;
mod helpers;
#[tokio::test]
async fn list_articles() {
    use realworld_db::entity::articles::Entity as Article;
    let db = test_db("test_create_user").await;
    let users: Vec<User> = create_users(&db, 5)
        .await
        .into_iter()
        .map(|(u, _)| u)
        .collect();
    let _articles = create_articles(&db, users).await;

    let results = Article::find()
        .all(&db.0.pool())
        .await
        .expect("Failed to get articles");

    assert_eq!(results.len(), 5);
}

#[tokio::test]
async fn delete_article() {
    let db = test_db("delete_article").await;
    let n_articles = 5;
    let users: Vec<User> = create_users(&db, n_articles)
        .await
        .into_iter()
        .map(|(u, _)| u)
        .collect();
    let articles = create_articles(&db, users).await;

    let article = articles[0].clone();

    let slug = article.slug.clone();

    db.0.delete_article(&article)
        .await
        .expect("Failed to delete article");

    let results = db.0.find_articles(Default::default()).await.unwrap();
    assert_eq!(results.len() as i32, n_articles - 1);

    let result = db.0.get_article_by_slug(&slug).await;
    assert!(result.is_err());
}
