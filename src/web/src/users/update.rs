use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use crate::errors::ApiResult;
use crate::extractor::User;
use crate::users::responses::UserResponse;
use crate::{auth::encode_token, context::ApplicationContext};
use domain::repositories::Repository;
use std::convert::{TryFrom, TryInto};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub user: UpdateUserRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}

impl TryFrom<UpdateUserRequest> for domain::UserUpdate {
    type Error = domain::PasswordError;

    fn try_from(u: UpdateUserRequest) -> Result<Self, Self::Error> {
        let update = Self {
            email: u.email,
            username: u.username,
            password: u
                .password
                .map(domain::Password::from_clear_text)
                .transpose()?,
            image: u.image,
            bio: u.bio,
        };
        Ok(update)
    }
}

pub async fn update_user(
    ctx: Extension<ApplicationContext>,
    user: User,
    request: Json<Request>,
) -> ApiResult<Json<UserResponse>> {
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;
    let updated_user = user.update(request.0.user.try_into()?, ctx.repo()).await?;
    let token = encode_token(updated_user.id);

    let response = UserResponse::from((updated_user, token));

    Ok(response.into())
}
