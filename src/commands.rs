use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{RoleRepository, UserRepository};

use diesel::{Connection, PgConnection};
use std::str::FromStr;

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to postgres")
}

pub fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection();
    let password_hash = auth::hash_password(password).unwrap();

    let new_user = NewUser {
        username,
        password: password_hash,
    };
    let role_codes = role_codes
        .iter()
        .map(|v| RoleCode::from_str(&v).unwrap())
        .collect();

    let user = UserRepository::create(&mut c, new_user, role_codes).unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).unwrap();

    println!("Role assigned {:?}", roles);
}

pub fn list_users() {
    let mut c = load_db_connection();

    let users = UserRepository::find_with_roles(&mut c).unwrap();
    for user in users {
        println!("{:?}", user)
    }
}

pub fn delete_user(id: i32) {
    let mut c = load_db_connection();

    UserRepository::delete(&mut c, id).unwrap();
}
