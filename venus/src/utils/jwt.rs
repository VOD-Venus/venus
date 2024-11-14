use std::sync::LazyLock;

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};

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
        let key = "xfy";
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
