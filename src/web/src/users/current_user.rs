use axum::{Extension, Json};
use domain::repositories::Repository;

use crate::{auth::encode_token, context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::UserResponse;

pub async fn get_current_user(
    ctx: Extension<ApplicationContext>,
    user: User,
) -> ApiResult<Json<UserResponse>> {
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;
    let token = encode_token(user.id);

    let payload: UserResponse = (user, token).into();

    Ok(payload.into())
}
