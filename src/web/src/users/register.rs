use axum::{Extension, Json};
use domain::{repositories::Repository, SignUp};
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

use crate::{auth::encode_token, context::ApplicationContext, errors::ApiResult};

use super::responses::UserResponse;

#[derive(Deserialize, Debug)]
pub struct RegistrationRequest {
    user: NewUserRequest,
}

#[derive(Deserialize, Debug)]
pub struct NewUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<RegistrationRequest> for SignUp {
    type Error = domain::PasswordError;

    fn try_from(r: RegistrationRequest) -> Result<Self, Self::Error> {
        let sign_up = Self {
            username: r.user.username,
            password: domain::Password::from_clear_text(r.user.password)?,
            email: r.user.email,
        };
        Ok(sign_up)
    }
}

pub async fn register(
    ctx: Extension<ApplicationContext>,
    request: Json<RegistrationRequest>,
) -> ApiResult<Json<UserResponse>> {
    let sign_up: SignUp = request.0.try_into()?;
    let new_user = ctx.repo().sign_up(sign_up).await?;
    let token = encode_token(new_user.id);

    Ok(UserResponse::from((new_user, token)).into())
}
