use axum::{Extension, Json};
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::ArticleResponse;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: NewArticleRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewArticleRequest {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Option<Vec<String>>,
}

impl From<NewArticleRequest> for domain::ArticleContent {
    fn from(a: NewArticleRequest) -> domain::ArticleContent {
        domain::ArticleContent {
            title: a.title,
            description: a.description,
            body: a.body,
            tag_list: a.tag_list.unwrap_or_default(),
        }
    }
}

pub async fn insert_article(
    ctx: Extension<ApplicationContext>,
    user: User,
    request: Json<Request>,
) -> ApiResult<Json<ArticleResponse>> {
    let author = ctx.repo().get_user_by_id(user.user_id()).await?;
    let published_article = author.publish(request.0.article.into(), ctx.repo()).await?;

    Ok(ArticleResponse::from(published_article).into())
}
