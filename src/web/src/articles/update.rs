use axum::{extract::Path, Extension, Json};
use domain::{repositories::Repository, ArticleUpdate};
use serde::{Deserialize, Serialize};

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::ArticleResponse;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub article: UpdateArticleRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticleRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
}

impl From<Request> for ArticleUpdate {
    fn from(r: Request) -> ArticleUpdate {
        ArticleUpdate {
            title: r.article.title,
            body: r.article.body,
            description: r.article.description,
        }
    }
}

pub async fn update_article(
    ctx: Extension<ApplicationContext>,
    user: User,
    Path(slug): Path<String>,
    request: Json<Request>,
) -> ApiResult<Json<ArticleResponse>> {
    let article = ctx.repo().get_article_by_slug(&slug).await?;
    let user = ctx.repo().get_user_by_id(user.user_id()).await?;
    let updated_article = user
        .update_article(article, request.0.into(), ctx.repo())
        .await?;

    let response: ArticleResponse = ctx
        .repo()
        .get_article_view(&user, updated_article)
        .await?
        .into();

    Ok(response.into())
}
