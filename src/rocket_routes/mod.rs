use diesel::PgConnection;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
pub mod crates;
pub mod rustaceans;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

pub fn not_found_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::NotFound, json!("Error"))
}

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}
