mod helpers;

use chrono::Utc;
use helpers::{create_article, create_user, create_users};
use realworld_domain::repositories::Repository;
use realworld_domain::{Article, ArticleMetadata, Profile};

use crate::helpers::generate::article_content;
use realworld_tests::db::test_db;

#[tokio::test]
async fn you_cannot_favorite_an_article_which_does_not_exist() {
    let db = test_db("you_cannot_favorite_an_article_which_does_not_exist").await;
    let user = create_user(&db).await.0;
    // Slug not pointing to any article in the DB
    let article_slug = "hello";

    let result =
        db.0.favorite(
            &Article {
                slug: article_slug.to_string(),
                content: article_content(),
                author: Profile {
                    username: String::default(),
                    bio: None,
                    image: None,
                },
                metadata: ArticleMetadata {
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                },
                favorites_count: 0,
            },
            &user,
        )
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn you_can_favorite_an_article_twice_but_it_only_counts_for_one() {
    let db = test_db("you_can_favorite_an_article_twice_but_it_only_counts_for_one").await;

    let user = create_user(&db).await.0;
    let article = create_article(&db, &user).await;

    let result = db.0.favorite(&article, &user).await;
    assert!(result.is_ok());

    let result = db.0.favorite(&article, &user).await;
    assert!(result.is_ok());

    assert_eq!(1, db.0.n_favorites(&article).await.unwrap());
    assert!(db.0.is_favorite(&article, &user).await.unwrap());
}

#[tokio::test]
async fn you_can_favorite_an_article_which_you_never_favorited() {
    let db = test_db("you_can_favorite_an_article_which_you_never_favorited").await;

    let user = create_user(&db).await.0;
    let article = create_article(&db, &user).await;

    let result = db.0.unfavorite(&article, &user).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn favorites_works() {
    let db = test_db("favorites_works").await;

    let author = create_user(&db).await.0;
    let article = create_article(&db, &author).await;

    let n_fans = 10;
    let fans = create_users(&db, n_fans).await;

    for (fan, _) in &fans {
        assert!(!db.0.is_favorite(&article, &fan).await.unwrap());
        db.0.favorite(&article, &fan)
            .await
            .expect("Failed to fav article");
        assert!(db.0.is_favorite(&article, &fan).await.unwrap());
    }

    assert_eq!(n_fans as i64, db.0.n_favorites(&article).await.unwrap());

    for (fan, _) in &fans {
        db.0.unfavorite(&article, &fan)
            .await
            .expect("Failed to fav article");
    }

    assert_eq!(0, db.0.n_favorites(&article).await.unwrap());
}
