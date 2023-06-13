use diesel::result::Error::NotFound;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{NewRustacean, Rustacean};
use crate::repositories::RustaceanRepository;
use crate::rocket_routes::{not_found_error, server_error, DbConn};

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustaceans(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    new_rustacean: Json<NewRustacean>,
    db: DbConn,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustacean| Custom(Status::Created, json!(rustacean)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    id: i32,
    rustacean: Json<Rustacean>,
    db: DbConn,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::update(c, id, rustacean.into_inner())
            .map(|rustacean| json!(rustacean))
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustaceans(id: i32, db: DbConn) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}
