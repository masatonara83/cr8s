use super::{not_found_error, server_error, CacheConn, DbConn};
use crate::{auth, models::User, repositories::UserRepository};
use diesel::result::Error::NotFound;
use rocket::{http::Status, response::status::Custom, serde::json::Json};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use serde_json::{json, Value};

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    credentials: Json<auth::Credentials>,
    db: DbConn,
    mut cache: Connection<CacheConn>,
) -> Result<Value, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |c| {
            UserRepository::find_by_username(c, &username).map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
        })
        .await?;

    let session_id = auth::autorize_user(&user, &credentials)
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| json!({ "token": session_id }))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/me")]
pub fn me(user: User) -> Value {
    json!(user)
}
