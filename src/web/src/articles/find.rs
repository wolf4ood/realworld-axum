use axum::{extract::Path, Extension, Json};
use domain::repositories::Repository;

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::ArticleResponse;

pub async fn get_article(
    ctx: Extension<ApplicationContext>,
    Path(slug): Path<String>,
    user: Option<User>,
) -> ApiResult<Json<ArticleResponse>> {
    let article = ctx.repo().get_article_by_slug(&slug).await?;

    match user {
        Some(user) => {
            let user = ctx.repo().get_user_by_id(user.user_id()).await?;
            let article_view = ctx.repo().get_article_view(&user, article).await?;
            Ok(ArticleResponse::from(article_view).into())
        }
        None => Ok(ArticleResponse::from(article).into()),
    }
}
