use std::env;
use easy_password::bcrypt::{hash_password, verify_password, PasswordError};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use chrono::Utc;
use jsonwebtoken::errors::{Error, ErrorKind};

use crate::{models::User, schema::users::username};

const JWT_SECRET: &[u8] = b"secret"; 

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash(password: &str) -> Result<String, PasswordError> {
    let SECRET_KEY: String = env::var("SECRET_KEY").expect("no SECRET_KEY"); 

    hash_password(password, SECRET_KEY.as_bytes(), 12).map_err(|err| {
        err
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, PasswordError> {
    let SECRET_KEY: String = env::var("SECRET_KEY").expect("no SECRET_KEY"); 

    verify_password(password, hash, SECRET_KEY.as_bytes()).map_err(|err| {
        err
    })
}

pub fn create_jwt(user_name: &str) -> Result<String, ErrorKind> {

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let header = Header::new(Algorithm::HS512);
    encode(&header, &user_name, &EncodingKey::from_secret(&JWT_SECRET))
        .map_err(|_| ErrorKind::InvalidToken)
}