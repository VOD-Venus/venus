use std::sync::LazyLock;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = Alphanumeric.sample_string(&mut rand::thread_rng(), 32);
    Keys::new(secret.as_bytes())
});

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    // aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize,  // Optional. Not Before (as UTC timestamp)
    pub sub: String, // Optional. Subject (whom token refers to)
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::InvalidToken("Extract the token failed".into()))?;
        // Decode the user data
        let token_data = decode_jwt(bearer.token())
            .map_err(|_| AppError::InvalidToken("Deocde the token failed".into()))?;

        Ok(token_data.claims)
    }
}

pub fn encode_jwt(claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&Header::new(Algorithm::HS256), &claims, &KEYS.encoding)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(token, &KEYS.decoding, &Validation::default())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn jwt_works() {
        let now = SystemTime::now();
        let seconds_since_epoch: usize = now
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap();
        let sub = "xfy".to_string();
        let claims = Claims {
            exp: seconds_since_epoch + 1000,
            iat: seconds_since_epoch,
            sub: sub.clone(),
        };
        let jwt = encode_jwt(&claims).unwrap();

        let token_data = decode_jwt(&jwt).unwrap();
        println!("{token_data:?}");

        assert_eq!(token_data.header.alg, Algorithm::HS256);
        assert_eq!(token_data.claims.sub, sub);
    }
}
