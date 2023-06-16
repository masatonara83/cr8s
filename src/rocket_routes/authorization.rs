use diesel::result::Error::NotFound;
use rocket::{response::status::Custom, serde::json::Json};
use serde_json::{json, Value};

use crate::{auth, repositories::UserRepository};

use super::{not_found_error, server_error, DbConn};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<auth::Credentials>,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        UserRepository::find_by_username(c, &credentials.username)
            .map(|user| {
                if let Ok(token) = auth::autorize_user(&user, &credentials) {
                    return json!(token);
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
