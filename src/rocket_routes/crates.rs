use diesel::result::Error::NotFound;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::{serde_json::json, Json, Value};

use crate::models::{Crate, NewCrate, User};
use crate::repositories::CrateRepository;
use crate::rocket_routes::{not_found_error, server_error, DbConn, EditUser};

#[rocket::get("/crates")]
pub async fn get_crates(db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        CrateRepository::find_multiple(c, 100)
            .map(|crates| json!(crates))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::get("/crates/<id>")]
pub async fn view_crates(id: i32, db: DbConn, _user: User) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::find(c, id)
            .map(|a_crate| json!(a_crate))
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    new_crate: Json<NewCrate>,
    db: DbConn,
    _user: EditUser,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::create(c, new_crate.into_inner())
            .map(|a_crate| Custom(Status::Created, json!(a_crate)))
            .map_err(|e| server_error(e.into()))
    })
    .await
}

#[rocket::put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    id: i32,
    a_crate: Json<Crate>,
    db: DbConn,
    _user: EditUser,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::update(c, id, a_crate.into_inner())
            .map(|a_crate| json!(a_crate))
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(
    id: i32,
    db: DbConn,
    _user: EditUser,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepository::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| match e {
                NotFound => not_found_error(e.into()),
                _ => server_error(e.into()),
            })
    })
    .await
}
