use diesel::PgConnection;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use rocket::Request;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use rocket_db_pools::{deadpool_redis, Database};
use rocket_sync_db_pools::database;

use crate::models::{RoleCode, User};
use crate::repositories::{RoleRepository, UserRepository};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(deadpool_redis::Pool);

pub fn not_found_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::NotFound, json!("Not Found Error"))
}

pub fn server_error(e: Box<dyn std::error::Error>) -> Custom<Value> {
    log::error!("{}", e);
    Custom(Status::InternalServerError, json!("Server Error"))
}

pub struct EditUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditUser {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = request
            .guard::<User>()
            .await
            .expect("Cannot retrieve logged in user in request guard");
        let db = request
            .guard::<DbConn>()
            .await
            .expect("Cannot connect to postgres in request guard");
        let editor_option = db
            .run(|c| match RoleRepository::find_by_user(c, &user) {
                Ok(roles) => {
                    log::info!("Assigned roles {:?}", roles);
                    let is_editor = roles.iter().any(|r| match r.code {
                        RoleCode::Admin => true,
                        RoleCode::Editor => true,
                        _ => false,
                    });
                    log::info!("Is Editor is {:?}", is_editor);
                    is_editor.then_some(EditUser(user))
                }
                _ => None,
            })
            .await;
        match editor_option {
            Some(editor) => Outcome::Success(editor),
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        //Authrization: Bearer SESSION_ID_128_CHARS_LONG
        let session_header = request
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        //session_headerが存在するかチェック
        if let Some(session_value) = session_header {
            //redisへの接続確認
            let mut cache = request
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Cannot connect to redis in request guard");
            //postgresへの接続確認
            let db = request
                .guard::<DbConn>()
                .await
                .expect("Cannot connect to postgres in request guard");
            let result = cache
                .get::<_, i32>(format!("sessions/{}", session_value[1]))
                .await;
            if let Ok(user_id) = result {
                return match db.run(move |c| UserRepository::find(c, user_id)).await {
                    Ok(user) => Outcome::Success(user),
                    _ => Outcome::Failure((Status::Unauthorized, ())),
                };
            }
        }

        Outcome::Failure((Status::Unauthorized, ()))
    }
}
