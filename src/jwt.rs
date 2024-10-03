use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation, TokenData, errors::Error};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: usize,
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, Error> {
    let secret = env::var("AUTH0_SECRET").expect("AUTH0_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation)
}
