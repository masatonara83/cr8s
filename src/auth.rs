use argon2::password_hash::Error;
use argon2::password_hash::SaltString;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::Rng;
use serde::Deserialize;

use crate::models::User;

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub fn autorize_user(user: &User, credentials: &Credentials) -> Result<String, Error> {
    //ハッシュ化での比較
    let db_hash = PasswordHash::new(&user.password)?;
    let argon = Argon2::default();
    argon.verify_password(credentials.password.as_bytes(), &db_hash)?;

    Ok(rand::thread_rng()
        .sample_iter(&Alphanumeric) //0-9a-z
        .take(128) //128文字を指定
        .map(char::from)
        .collect())
}

pub fn hash_password(password: String) -> Result<String, Error> {
    //passwordnのハッシュ化
    let salt = SaltString::generate(OsRng);
    let argon = Argon2::default();
    let password_hash = argon.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
