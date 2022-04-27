use axum::{extract::Path, Extension, Json};
use domain::repositories::Repository;
use futures::{future::join_all, FutureExt};

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::CommentsResponse;

pub async fn get(
    ctx: Extension<ApplicationContext>,
    user: Option<User>,
    Path(slug): Path<String>,
) -> ApiResult<Json<CommentsResponse>> {
    let article = ctx.repo().get_article_by_slug(&slug).await?;
    let comments = article.comments(ctx.repo()).await?;

    let response: CommentsResponse = match user {
        Some(user) => {
            let user = ctx.repo().get_user_by_id(user.user_id()).await?;
            let result: Vec<_> = comments
                .into_iter()
                .map(|c| c.view(&user, ctx.repo()).boxed())
                .collect();
            let comment_views: Result<Vec<_>, _> = join_all(result).await.into_iter().collect();
            CommentsResponse::from(comment_views?)
        }
        None => CommentsResponse::from(comments),
    };

    Ok(response.into())
}
