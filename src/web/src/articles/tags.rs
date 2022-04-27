use axum::{Extension, Json};
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};

use crate::{context::ApplicationContext, errors::ApiResult};

#[derive(Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

pub async fn tags(ctx: Extension<ApplicationContext>) -> ApiResult<Json<TagsResponse>> {
    let tags = ctx.repo().get_tags().await?;
    let response = TagsResponse {
        tags: tags.into_iter().collect(),
    };

    Ok(response.into())
}
