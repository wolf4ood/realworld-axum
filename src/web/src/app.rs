use crate::{articles, comments, context::ApplicationContext, profiles, users};
use axum::{
    http::Method,
    routing::{delete, get, post},
    Extension, Router,
};
use domain::repositories::Repository;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn get_app<R: Repository + Send + Sync + 'static>(repository: R) -> Router {
    Router::new()
        .nest("/api", api())
        .layer(Extension(ApplicationContext::new(repository)))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST]),
        )
        .layer(TraceLayer::new_for_http())
}

pub fn api() -> Router {
    Router::new()
        .route(
            "/user",
            get(users::get_current_user).put(users::update_user),
        )
        .route("/users", post(users::register))
        .route("/users/login", post(users::login))
        .route("/profiles/:username", get(profiles::get_profile))
        .route(
            "/profiles/:username/follow",
            post(profiles::follow).delete(profiles::unfollow),
        )
        .route("/tags", get(articles::tags))
        .route(
            "/articles",
            get(articles::list_articles).post(articles::insert_article),
        )
        .route("/articles/feed", get(articles::feed))
        .route(
            "/articles/:slug",
            get(articles::get_article)
                .put(articles::update_article)
                .delete(articles::delete_article),
        )
        .route(
            "/articles/:slug/comments",
            get(comments::get).post(comments::create),
        )
        .route("/articles/:slug/comments/:id", delete(comments::delete))
        .route(
            "/articles/:slug/favorite",
            post(articles::favorite).delete(articles::unfavorite),
        )
}
