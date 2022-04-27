use axum::{extract::Query, Extension, Json};
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::ArticlesResponse;

#[derive(Serialize, Deserialize)]
pub struct FeedQuery {
    #[serde(default)]
    pub limit: u64,

    #[serde(default)]
    pub offset: u64,
}

impl Default for FeedQuery {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
        }
    }
}

impl From<FeedQuery> for domain::FeedQuery {
    fn from(f: FeedQuery) -> Self {
        Self {
            limit: f.limit,
            offset: f.offset,
        }
    }
}

pub async fn feed(
    ctx: Extension<ApplicationContext>,
    user: User,
    Query(query): Query<FeedQuery>,
) -> ApiResult<Json<ArticlesResponse>> {
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;

    let articles = user.feed(query.into(), ctx.repo()).await?;
    let response = ArticlesResponse::from(articles);

    Ok(response.into())
}
