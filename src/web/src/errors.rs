//! A sub-module to prescribe how each domain error gets converted to an HTTP response.
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::{
    ChangeArticleError, DatabaseError, DeleteCommentError, GetArticleError, GetUserError,
    LoginError, PasswordError, PublishArticleError, SignUpError,
};
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Login(#[from] LoginError),
    #[error(transparent)]
    ChangeArticle(#[from] ChangeArticleError),
    #[error(transparent)]
    Database(#[from] DatabaseError),
    #[error(transparent)]
    DeleteComment(#[from] DeleteCommentError),
    #[error(transparent)]
    GetArticle(#[from] GetArticleError),
    #[error(transparent)]
    GetUser(#[from] GetUserError),
    #[error(transparent)]
    Password(#[from] PasswordError),
    #[error(transparent)]
    PublishArticle(#[from] PublishArticleError),
    #[error(transparent)]
    SingUp(#[from] SignUpError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Error on request: {}", self);
        let (status, error_message) = match self {
            ApiError::Login(LoginError::NotFound) => not_found(""),
            ApiError::Login(LoginError::PasswordError(_)) => bad_request(""),
            ApiError::Login(LoginError::DatabaseError(_)) => {
                internal_server_error("Something went wrong")
            }
            ApiError::ChangeArticle(ChangeArticleError::ArticleNotFound { .. }) => not_found(""),
            ApiError::ChangeArticle(ChangeArticleError::Forbidden { .. }) => unauthorized(""),
            ApiError::ChangeArticle(ChangeArticleError::DatabaseError { .. }) => {
                internal_server_error("Something went wrong")
            }
            ApiError::Database(_) => internal_server_error("Something went wrong"),
            ApiError::DeleteComment(DeleteCommentError::CommentNotFound { .. }) => not_found(""),
            ApiError::DeleteComment(DeleteCommentError::UserNotFound { .. }) => not_found(""),
            ApiError::DeleteComment(DeleteCommentError::Forbidden { .. }) => unauthorized(""),
            ApiError::DeleteComment(DeleteCommentError::DatabaseError { .. }) => {
                internal_server_error("Something went wrong")
            }
            ApiError::GetArticle(GetArticleError::ArticleNotFound { .. }) => not_found(""),
            ApiError::GetArticle(GetArticleError::AuthorNotFound { .. }) => not_found(""),
            ApiError::GetArticle(GetArticleError::DatabaseError(_)) => {
                internal_server_error("Something went wrong")
            }
            ApiError::GetUser(GetUserError::NotFound { .. }) => not_found(""),
            ApiError::GetUser(GetUserError::NotFoundByUsername { .. }) => not_found(""),
            ApiError::GetUser(GetUserError::DatabaseError { .. }) => {
                internal_server_error("Something went wrong")
            }
            ApiError::Password(_) => bad_request(""),
            ApiError::PublishArticle(PublishArticleError::AuthorNotFound { .. }) => not_found(""),
            ApiError::PublishArticle(PublishArticleError::DatabaseError { .. }) => {
                internal_server_error("Something went wrong")
            }
            ApiError::PublishArticle(PublishArticleError::DuplicatedSlug { .. }) => {
                bad_request("Invalid slug")
            }
            ApiError::SingUp(_) => internal_server_error("Something went wrong"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

fn internal_server_error(msg: &str) -> (StatusCode, &str) {
    (StatusCode::INTERNAL_SERVER_ERROR, msg)
}
fn bad_request(msg: &str) -> (StatusCode, &str) {
    (StatusCode::BAD_REQUEST, msg)
}
fn not_found(msg: &str) -> (StatusCode, &str) {
    (StatusCode::NOT_FOUND, msg)
}
fn unauthorized(msg: &str) -> (StatusCode, &str) {
    (StatusCode::UNAUTHORIZED, msg)
}
