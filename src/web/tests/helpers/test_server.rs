use std::path::PathBuf;

use application::configuration::Settings;
use axum::http::Request;
use axum::response::Response;
use axum::Router;
use realworld_web::get_app;
use realworld_web::users::responses::UserResponse;

use domain::articles::ArticleQuery;
use domain::SignUp;
use realworld_web::articles::responses::{ArticleResponse, ArticlesResponse};
use realworld_web::comments::responses::{CommentResponse, CommentsResponse};
use realworld_web::profiles::responses::ProfileResponse;
use serde::de::DeserializeOwned;
use serde_json::json;
use tower::ServiceExt;

use super::test_db::Db;

pub struct TestApp {
    pub server: Router,
    pub repository: Db,
}
pub async fn test_db(name: &str) -> Db {
    let settings = Settings::new(PathBuf::from("../../")).expect("Failed to load configuration");
    Db::create(settings.database, name).await
}

impl TestApp {
    pub async fn create(name: &str) -> Self {
        let db = test_db(name).await;
        let app = get_app(db.0.clone());
        Self {
            server: app,
            repository: db,
        }
    }

    pub async fn register_user(
        &mut self,
        user: &SignUp,
        password: &str,
    ) -> Result<UserResponse, Response> {
        let response = self
            .server
            .clone()
            .oneshot(
                Request::post("/api/users")
                    .header("Content-Type", "application/json")
                    .body(
                        json!({
                            "user": {
                                "email": user.email,
                                "password": password,
                                "username": user.username,
                            }
                        })
                        .to_string()
                        .into_bytes()
                        .into(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn login_user(
        &mut self,
        email: &str,
        password: &str,
    ) -> Result<UserResponse, Response> {
        let response = self
            .server
            .clone()
            .oneshot(
                Request::post("/api/users/login")
                    .header("Content-Type", "application/json")
                    .body(
                        json!({
                            "user": {
                                "email": email.to_owned(),
                                "password": password.to_owned(),
                            }
                        })
                        .to_string()
                        .into_bytes()
                        .into(),
                    )
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_current_user(&mut self, token: &String) -> Result<UserResponse, Response> {
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::get("/api/user")
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn update_user_details(
        &mut self,
        details: &realworld_web::users::update::Request,
        token: &String,
    ) -> Result<UserResponse, Response> {
        let response = self
            .server
            .clone()
            .oneshot(
                Request::put("/api/user")
                    .header("Authorization", format!("token: {}", token))
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(details).unwrap().into_bytes().into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn create_article(
        &mut self,
        article: &realworld_web::articles::insert::Request,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let body = serde_json::to_string(article).unwrap();
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::post("/api/articles")
                    .header("Authorization", auth_header)
                    .header("Content-Type", "application/json")
                    .body(body.into_bytes().into())
                    .unwrap(),
            )
            .await
            .unwrap();

        response_json_if_success(response).await
    }

    pub async fn update_article(
        &mut self,
        article: &realworld_web::articles::update::Request,
        slug: &str,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}", slug);
        let body = serde_json::to_string(article).unwrap();
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::put(url)
                    .header("Authorization", auth_header)
                    .header("Content-Type", "application/json")
                    .body(body.into_bytes().into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_articles(
        &mut self,
        query: Option<ArticleQuery>,
    ) -> Result<ArticlesResponse, Response> {
        let query_string = serde_qs::to_string(&query).unwrap();
        let url = format!("/api/articles?{}", query_string);
        let response = self
            .server
            .clone()
            .oneshot(Request::get(url).body("".into()).unwrap())
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_article(
        &mut self,
        slug: &str,
        token: Option<&str>,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}", slug);
        let request = match token {
            Some(token) => {
                let auth_header = format!("token: {}", token);
                Request::get(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap()
            }
            None => Request::get(url).body("".into()).unwrap(),
        };
        let response = self.server.clone().oneshot(request).await.unwrap();
        response_json_if_success(response).await
    }

    pub async fn delete_article(&mut self, slug: &str, token: &str) -> Result<(), Response> {
        let url = format!("/api/articles/{}", slug);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::delete(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response)
        }
    }

    pub async fn favorite_article(
        &mut self,
        slug: &str,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}/favorite", slug);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::post(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn unfavorite_article(
        &mut self,
        slug: &str,
        token: &str,
    ) -> Result<ArticleResponse, Response> {
        let url = format!("/api/articles/{}/favorite", slug);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::delete(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_profile(
        &mut self,
        username: &str,
        token: Option<&str>,
    ) -> Result<ProfileResponse, Response> {
        let url = format!("/api/profiles/{}", username);
        let request = match token {
            Some(token) => {
                let auth_header = format!("token: {}", token);
                Request::get(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap()
            }
            None => Request::get(url).body("".into()).unwrap(),
        };
        let response = self.server.clone().oneshot(request).await.unwrap();
        response_json_if_success(response).await
    }

    pub async fn follow_profile(
        &mut self,
        username: &str,
        token: &str,
    ) -> Result<ProfileResponse, Response> {
        let url = format!("/api/profiles/{}/follow", username);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::post(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn unfollow_profile(
        &mut self,
        username: &str,
        token: &str,
    ) -> Result<ProfileResponse, Response> {
        let url = format!("/api/profiles/{}/follow", username);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::delete(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn get_comments(
        &mut self,
        slug: &str,
        token: Option<&str>,
    ) -> Result<CommentsResponse, Response> {
        let url = format!("/api/articles/{}/comments", slug);
        let request = match token {
            Some(token) => {
                let auth_header = format!("token: {}", token);
                Request::get(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap()
            }
            None => Request::get(url).body("".into()).unwrap(),
        };
        let response = self.server.clone().oneshot(request).await.unwrap();
        response_json_if_success(response).await
    }

    pub async fn create_comment(
        &mut self,
        slug: &str,
        comment: &realworld_web::comments::create::Request,
        token: &str,
    ) -> Result<CommentResponse, Response> {
        let url = format!("/api/articles/{}/comments", slug);
        let auth_header = format!("token: {}", token);
        let body = serde_json::to_string(comment).unwrap();
        let response = self
            .server
            .clone()
            .oneshot(
                Request::post(url)
                    .header("Authorization", auth_header)
                    .header("Content-Type", "application/json")
                    .body(body.into_bytes().into())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json_if_success(response).await
    }

    pub async fn delete_comment(
        &mut self,
        slug: &str,
        comment_id: &u64,
        token: &str,
    ) -> Result<(), Response> {
        let url = format!("/api/articles/{}/comments/{}", slug, comment_id);
        let auth_header = format!("token: {}", token);
        let response = self
            .server
            .clone()
            .oneshot(
                Request::delete(url)
                    .header("Authorization", auth_header)
                    .body("".into())
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status().is_success() {
            Ok(())
        } else {
            Err(response)
        }
    }
}

impl std::ops::Drop for TestApp {
    fn drop(&mut self) {}
}

pub async fn response_json_if_success<T: DeserializeOwned>(
    response: Response,
) -> Result<T, Response> {
    if response.status().is_success() {
        Ok(response_json(response).await)
    } else {
        Err(response)
    }
}

pub async fn response_json<T: DeserializeOwned>(res: Response) -> T {
    let body = hyper::body::to_bytes(res.into_body()).await.unwrap();

    serde_json::from_slice(&body).expect("Could not parse body.")
}
