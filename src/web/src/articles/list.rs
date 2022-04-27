use axum::{extract::Query, Extension, Json};
use domain::repositories::Repository;
use serde::Deserialize;

use crate::{context::ApplicationContext, errors::ApiResult, extractor::User};

use super::responses::ArticlesResponse;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct ArticleQuery {
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub tag: Option<String>,
}

impl From<ArticleQuery> for domain::ArticleQuery {
    fn from(q: ArticleQuery) -> Self {
        Self {
            author: q.author,
            favorited: q.favorited,
            tag: q.tag,
        }
    }
}

pub async fn list_articles(
    ctx: Extension<ApplicationContext>,
    Query(query): Query<ArticleQuery>,
    user: Option<User>,
) -> ApiResult<Json<ArticlesResponse>> {
    let articles = ctx.repo().find_articles(query.into()).await?;

    match user {
        Some(user) => {
            let user = ctx.repo().get_user_by_id(user.user_id()).await?;
            let views = ctx.repo().get_articles_views(&user, articles).await?;
            Ok(ArticlesResponse::from(views).into())
        }
        None => Ok(ArticlesResponse::from(articles).into()),
    }
}
