use axum::{extract::Path, Extension, Json};
use domain::repositories::Repository;

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::ProfileResponse;

pub async fn get_profile(
    ctx: Extension<ApplicationContext>,
    user: Option<User>,
    Path(username): Path<String>,
) -> ApiResult<Json<ProfileResponse>> {
    let response: ProfileResponse = match user {
        Some(user) => {
            let user = ctx.repo().get_user_by_id(user.user_id()).await?;
            let view = ctx.repo().get_profile_view(&user, &username).await?;
            ProfileResponse::from(view)
        }
        None => {
            let profile = ctx.repo().get_profile(&username).await?;
            ProfileResponse::from(profile)
        }
    };
    Ok(response.into())
}
