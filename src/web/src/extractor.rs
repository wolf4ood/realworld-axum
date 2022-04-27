use crate::auth::{extract_claims, Claims};
use axum::extract::FromRequest;
use axum::http::StatusCode;
use uuid::Uuid;

pub struct User(Claims);

impl User {
    pub fn user_id(&self) -> Uuid {
        self.0.user_id()
    }
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        extract_claims(req.headers())
            .ok_or((StatusCode::UNAUTHORIZED, "Unauthorized"))
            .map(User)
    }
}
