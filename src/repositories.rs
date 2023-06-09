use diesel::prelude::*;

use crate::models::{Crate, NewCrate, NewRustacean, Rustacean};
use crate::schema::{crates, rustaceans};

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(conn: &mut PgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(conn)
    }

    pub fn find_multiple(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(conn)
    }

    pub fn create(conn: &mut PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(conn)
    }

    pub fn update(
        conn: &mut PgConnection,
        id: i32,
        rustacean: Rustacean,
    ) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email),
            ))
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(conn)
    }
}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find(conn: &mut PgConnection, id: i32) -> QueryResult<Crate> {
        crates::table.find(id).get_result(conn)
    }

    pub fn find_multiple(conn: &mut PgConnection, limit: i64) -> QueryResult<Vec<Crate>> {
        crates::table.limit(limit).load(conn)
    }

    pub fn create(conn: &mut PgConnection, new_crate: NewCrate) -> QueryResult<Crate> {
        diesel::insert_into(crates::table)
            .values(new_crate)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, id: i32, a_crate: Crate) -> QueryResult<Crate> {
        diesel::update(crates::table.find(id))
            .set((
                crates::rustacean_id.eq(a_crate.rustacean_id),
                crates::code.eq(a_crate.code),
                crates::name.eq(a_crate.name),
                crates::version.eq(a_crate.version),
                crates::description.eq(a_crate.description),
            ))
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(crates::table.find(id)).execute(conn)
    }
}
