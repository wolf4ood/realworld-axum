use crate::{
    context::ApplicationContext, errors::ApiResult, extractor::User,
    profiles::responses::ProfileResponse,
};
use axum::{extract::Path, Extension, Json};
use domain::repositories::Repository;

pub enum Action {
    Follow,
    Unfollow,
}

pub async fn follow(
    ctx: Extension<ApplicationContext>,
    user: User,
    Path(username): Path<String>,
) -> ApiResult<Json<ProfileResponse>> {
    _follow(&ctx.0, &user, &username, Action::Follow).await
}
pub async fn unfollow(
    ctx: Extension<ApplicationContext>,
    user: User,
    Path(username): Path<String>,
) -> ApiResult<Json<ProfileResponse>> {
    _follow(&ctx.0, &user, &username, Action::Unfollow).await
}

async fn _follow(
    ctx: &ApplicationContext,
    user: &User,
    username: &str,
    action: Action,
) -> ApiResult<Json<ProfileResponse>> {
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;
    let profile = ctx.repo().get_profile(username).await?;
    let view = match action {
        Action::Follow => user.follow(profile, ctx.repo()).await?,
        Action::Unfollow => user.unfollow(profile, ctx.repo()).await?,
    };

    let response = ProfileResponse::from(view);

    Ok(response.into())
}
