use jsonwebtoken::{
    decode, encode, errors, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub iat: u64,
    pub exp: u64,
}

pub fn encode_token(user_id: i32, jwt_secret: String) -> Result<String, errors::Error> {
    let now = SystemTime::now();
    let seconds_in_a_day = 86400;
    let seven_days_later = now + Duration::new(seconds_in_a_day * 7, 0);

    let now_as_seconds = now
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();
    let seven_days_later_as_secs = seven_days_later
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();

    let claims = Claims {
        sub: user_id,
        iat: now_as_seconds,
        exp: seven_days_later_as_secs,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

pub fn decode_token(token: String, jwt_secret: String) -> errors::Result<TokenData<Claims>> {
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
}
