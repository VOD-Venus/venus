use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    // aud: String, // Optional. Audience
    pub exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: usize, // Optional. Issued at (as UTC timestamp)
    // iss: String, // Optional. Issuer
    // nbf: usize,  // Optional. Not Before (as UTC timestamp)
    pub sub: String, // Optional. Subject (whom token refers to)
}

pub fn encode_jwt(claims: &Claims, key: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(key),
    )
}

pub fn decode_jwt(
    token: &str,
    key: &[u8],
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(key),
        &Validation::default(),
    )
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
        let jwt = encode_jwt(&claims, key.as_ref()).unwrap();

        let token_data = decode_jwt(&jwt, key.as_ref()).unwrap();
        println!("{token_data:?}");

        assert_eq!(token_data.header.alg, Algorithm::HS256);
        assert_eq!(token_data.claims.sub, sub);
    }
}
