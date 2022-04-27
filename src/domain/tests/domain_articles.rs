mod helpers;

use crate::helpers::{create_article, create_user, test_db};
use fake::fake;
use helpers::generate;
use realworld_domain::repositories::Repository as RepositoryTrait;
use realworld_domain::ArticleUpdate;

#[tokio::test]
async fn slugs_must_be_unique() {
    let db = test_db("slugs_must_be_unique").await;

    let author = create_user(&db).await.0;
    let first_draft = generate::article_content();
    let second_draft = first_draft.clone();
    // Two article drafts, with identical title => identical slug
    assert_eq!(first_draft.slug(), second_draft.slug());

    let _expected_slug = first_draft.slug();

    let result = author.publish(first_draft, &db.0).await;
    assert!(result.is_ok());

    // Publishing the second draft fails
    let result = author.publish(second_draft, &db.0).await;
    assert!(result.is_err());

    // With the appropriate error variant
    // TODO check the correct error
    // match result.unwrap_err() {
    //     PublishArticleError::DuplicatedSlug { slug, source: _ } => assert_eq!(expected_slug, slug),
    //     _ => panic!("Unexpected error"),
    // }
}

#[tokio::test]
async fn insert_and_retrieve_article() {
    let db = test_db("insert_and_retrieve_article").await;
    let author = create_user(&db).await.0;
    let author = db.0.get_user_by_id(author.id).await.unwrap();
    let draft = generate::article_content();

    let expected_article = author.publish(draft, &db.0).await.unwrap();
    let retrieved_article =
        db.0.get_article_by_slug(&expected_article.slug)
            .await
            .unwrap();
    assert_eq!(expected_article, retrieved_article);
}

#[tokio::test]
async fn update_and_retrieve_article() {
    let db = test_db("update_and_retrieve_article").await;

    let author = create_user(&db).await.0;
    let article = create_article(&db, &author).await;

    let update = ArticleUpdate {
        title: Some(fake!(Lorem.sentence(4, 10)).to_string()),
        description: Some(fake!(Lorem.paragraph(3, 10)).to_string()),
        body: Some(fake!(Lorem.paragraph(10, 5)).to_string()),
    };
    let updated_article = author
        .update_article(article, update.clone(), &db.0)
        .await
        .unwrap();

    assert_eq!(update.title, updated_article.content.title.into());
    assert_eq!(
        update.description,
        updated_article.content.description.into()
    );
    assert_eq!(update.body, updated_article.content.body.into());
}
