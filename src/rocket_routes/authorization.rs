use argon2::{PasswordHash, PasswordVerifier};
use diesel::result::Error::NotFound;
use rocket::{response::status::Custom, serde::json::Json};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::repositories::UserRepository;

use super::{not_found_error, server_error, DbConn};

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<Credentials>, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
            .map(|user| {
                //ハッシュ化での比較
                let db_hash = PasswordHash::new(&user.password).unwrap();
                let argon = argon2::Argon2::default();
                if argon
                    .verify_password(credentials.password.as_bytes(), &db_hash)
                    .is_ok()
                {
                    return json!("Success");
                }
                json!("Unauthorized")
            })
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}
