use axum::{Extension, Json};
use serde::Deserialize;

use crate::{auth::encode_token, context::ApplicationContext, errors::ApiResult};
use domain::repositories::Repository;

use super::responses::UserResponse;

#[derive(Deserialize)]
pub struct AuthRequest {
    user: AuthUser,
}

#[derive(Deserialize)]
pub struct AuthUser {
    email: String,
    password: String,
}

pub async fn login(
    ctx: Extension<ApplicationContext>,
    request: Json<AuthRequest>,
) -> ApiResult<Json<UserResponse>> {
    let logged_in_user = ctx
        .repo()
        .get_user_by_email_and_password(&request.user.email, &request.user.password)
        .await?;
    let token = encode_token(logged_in_user.id);

    let response = UserResponse::from((logged_in_user, token));

    Ok(response.into())
}
