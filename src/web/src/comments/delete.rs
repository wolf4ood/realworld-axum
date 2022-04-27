use axum::{extract::Path, Extension};
use domain::repositories::Repository;

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

pub async fn delete(
    ctx: Extension<ApplicationContext>,
    user: User,
    Path((_, comment_id)): Path<(String, u64)>,
) -> ApiResult<()> {
    let author = ctx.repo().get_user_by_id(user.user_id()).await?;
    let comment = ctx.repo().get_comment(comment_id).await?;
    author.delete_comment(comment, ctx.repo()).await?;

    Ok(())
}
