use axum::http::HeaderMap;
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;
use uuid::Uuid;

// TODO: get the secret from config
const SECRET: &str = "secret";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    sub: Uuid,
    exp: u64,
}

impl Claims {
    pub fn user_id(&self) -> Uuid {
        self.sub
    }
}

fn validation() -> Validation {
    Validation::default()
}

pub fn encode_token(sub: Uuid) -> String {
    encode(&Header::default(), &claims_for(sub, 3600), SECRET.as_ref()).unwrap()
}

pub fn claims_for(user_id: Uuid, expire_in: u64) -> Claims {
    Claims {
        sub: user_id,
        exp: seconds_from_now(expire_in),
    }
}

fn seconds_from_now(secs: u64) -> u64 {
    let expiry_time =
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap() + Duration::from_secs(secs);
    expiry_time.as_secs()
}

pub fn extract_token(headers: &HeaderMap) -> Option<&str> {
    match headers.get("Authorization") {
        Some(h) => match h.to_str() {
            Ok(hx) => hx.split(' ').nth(1),
            _ => None,
        },
        _ => None,
    }
}

pub fn extract_claims(headers: &HeaderMap) -> Option<Claims> {
    extract_token(headers).and_then(|token| {
        let decoded = decode::<Claims>(token, SECRET.as_ref(), &validation());
        if let Err(e) = &decoded {
            debug!("Failed to decode token {}", e);
        }
        decoded.map(|token_data| token_data.claims).ok()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn encode_decode_token() {
        let sub = Uuid::new_v4();
        let token = encode_token(sub);
        let decoded = decode::<Claims>(&token, "secret".as_ref(), &Validation::default());
        if let Err(e) = &decoded {
            println!("decode err: {}", e);
        }

        assert!(decoded.is_ok());
    }
}
