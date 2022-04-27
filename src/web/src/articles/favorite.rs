use axum::{extract::Path, Extension, Json};
use domain::repositories::Repository;

use super::responses::ArticleResponse;
use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

// pub async fn favorite<R: 'static + Repository + Sync + Send>(
//     cx: Request<Context<R>>,
// ) -> Result<Response, ErrorResponse> {
//     _favorite(cx, Action::Favorite).await
// }

pub async fn favorite(
    ctx: Extension<ApplicationContext>,
    Path(slug): Path<String>,
    user: User,
) -> ApiResult<Json<ArticleResponse>> {
    _favorite(&ctx, &user, &slug, Action::Favorite).await
}
pub async fn unfavorite(
    ctx: Extension<ApplicationContext>,
    Path(slug): Path<String>,
    user: User,
) -> ApiResult<Json<ArticleResponse>> {
    _favorite(&ctx, &user, &slug, Action::Unfavorite).await
}

pub enum Action {
    Favorite,
    Unfavorite,
}

pub async fn _favorite(
    ctx: &ApplicationContext,
    user: &User,
    slug: &str,
    action: Action,
) -> ApiResult<Json<ArticleResponse>> {
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;
    let article = ctx.repo().get_article_by_slug(slug).await?;

    let article_view = match action {
        Action::Favorite => user.favorite(article, ctx.repo()).await,
        Action::Unfavorite => user.unfavorite(article, ctx.repo()).await,
    }?;

    Ok(ArticleResponse::from(article_view).into())
}
