use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header, encode};
use password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::env;

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(user_id: &str) -> String {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret".to_string());

    let claims = Claims {
        sub: user_id.to_string(),
        exp: chrono::Utc::now().timestamp() as usize + 3600, // 1 hour expiry
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
