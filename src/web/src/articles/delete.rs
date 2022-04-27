use axum::{extract::Path, Extension};
use domain::repositories::Repository;

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

pub async fn delete_article(
    ctx: Extension<ApplicationContext>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<()> {
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;
    let article = ctx.repo().get_article_by_slug(&slug).await?;
    user.delete(article, ctx.repo()).await?;

    Ok(())
}
