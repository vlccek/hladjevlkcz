use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Algorithm, Header, TokenData, DecodingKey, Validation};
// 👈 New!
use std::env;
use std::error::Error;
use std::fmt::Debug;
use rocket::serde::{Deserialize, Serialize};

static VALIDITY: i64 = 60; // in seconds

#[derive(Serialize)]
struct Token {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
}


pub fn generate_token(login: String) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + VALIDITY,
        user: login,
    };

    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(include_bytes!("main.rs"))).unwrap()
}


fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(include_bytes!("main.rs")), &Validation::default())
}

fn verify_token(token_data: &TokenData<UserToken>, username :String) -> bool {
    todo!()//User::is_valid_login_session(&token_data.claims, conn)
}