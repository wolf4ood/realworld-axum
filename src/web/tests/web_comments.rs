// These tests are "integration" tests that exercise a workflow via the http service.

mod helpers;

use axum::http::StatusCode;
use helpers::test_server::TestApp;
use helpers::{create_article, create_user, create_users};

use fake::fake;
use itertools::Itertools;
use realworld_web::auth::encode_token;
use realworld_web::comments::create::NewCommentRequest;

#[tokio::test]
async fn comments_api() {
    let mut server = TestApp::create("comments_api").await;
    let user = create_user(&server.repository).await.0;
    let article = create_article(&server.repository, &user).await;
    let token = encode_token(user.id);

    let request = realworld_web::comments::create::Request {
        comment: NewCommentRequest {
            body: fake!(Lorem.paragraph(10, 5)),
        },
    };
    let first_comment = server
        .create_comment(&article.slug, &request, &token)
        .await
        .unwrap();
    assert_eq!(first_comment.comment.body, request.comment.body);
    assert_eq!(first_comment.comment.author.username, user.profile.username);
    assert_eq!(first_comment.comment.author.bio, user.profile.bio);
    assert_eq!(first_comment.comment.author.image, user.profile.image);
    // A user follows themselves, by definition
    assert_eq!(first_comment.comment.author.following, true);

    // A user can create more than one comment for the same article
    let request = realworld_web::comments::create::Request {
        comment: NewCommentRequest {
            body: fake!(Lorem.paragraph(10, 5)),
        },
    };
    let second_comment = server
        .create_comment(&article.slug, &request, &token)
        .await
        .unwrap();

    let comments = server
        .get_comments(&article.slug, Some(&token))
        .await
        .unwrap();
    assert_eq!(comments.comments.len(), 2);
    assert_eq!(comments.comments[0], first_comment.comment);
    assert_eq!(comments.comments[1], second_comment.comment);

    // Delete 1st comment
    server
        .delete_comment(&article.slug, &first_comment.comment.id, &token)
        .await
        .unwrap();
    let comments = server
        .get_comments(&article.slug, Some(&token))
        .await
        .unwrap();
    assert_eq!(comments.comments.len(), 1);
    assert_eq!(comments.comments[0], second_comment.comment);

    // Delete 2nd comment
    server
        .delete_comment(&article.slug, &second_comment.comment.id, &token)
        .await
        .unwrap();
    let comments = server
        .get_comments(&article.slug, Some(&token))
        .await
        .unwrap();
    assert_eq!(comments.comments.len(), 0);
}

#[tokio::test]
async fn you_cannot_delete_a_comment_which_you_did_not_write() {
    let mut server = TestApp::create("you_cannot_delete_a_comment_which_you_did_not_write").await;
    let mut users = create_users(&server.repository, 2)
        .await
        .into_iter()
        .map(|(u, _)| u)
        .collect_vec();
    let article_author = users.pop().unwrap();
    let comment_author = users.pop().unwrap();
    let article = create_article(&server.repository, &article_author).await;

    // comment_author write a comment
    let token = encode_token(comment_author.id);
    let request = realworld_web::comments::create::Request {
        comment: NewCommentRequest {
            body: fake!(Lorem.paragraph(10, 5)),
        },
    };
    let comment = server
        .create_comment(&article.slug, &request, &token)
        .await
        .unwrap();

    // article_author tries to delete it
    let token = encode_token(article_author.id);
    let response = server
        .delete_comment(&article.slug, &comment.comment.id, &token)
        .await;
    assert!(response.is_err());
    assert_eq!(StatusCode::UNAUTHORIZED, response.unwrap_err().status());
}
