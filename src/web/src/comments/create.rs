use axum::{extract::Path, Extension, Json};
use domain::{repositories::Repository, CommentContent};
use serde::{Deserialize, Serialize};

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::CommentResponse;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub comment: NewCommentRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewCommentRequest {
    pub body: String,
}

pub async fn create(
    ctx: Extension<ApplicationContext>,
    user: User,
    Path(slug): Path<String>,
    request: Json<Request>,
) -> ApiResult<Json<CommentResponse>> {
    let author = ctx.repo().get_user_by_id(user.user_id()).await?;
    let article = ctx.repo().get_article_by_slug(&slug).await?;
    let posted_comment = author
        .comment(&article, CommentContent(request.0.comment.body), ctx.repo())
        .await?;

    let response = CommentResponse {
        comment: posted_comment.into(),
    };
    Ok(response.into())
}
