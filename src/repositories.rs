use diesel::dsl::{now, IntervalDsl};
use diesel::prelude::*;

use crate::models::{
    Crate, NewCrate, NewRole, NewRustacean, NewUser, NewUserRole, Role, RoleCode, Rustacean, User,
    UserRole,
};
use crate::schema::{crates, roles, rustaceans, users, users_roles};

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
        crates::table.limit(limit).load::<Crate>(conn)
    }

    pub fn find_since(conn: &mut PgConnection, hours_since: i32) -> QueryResult<Vec<Crate>> {
        crates::table
            .filter(crates::created_at.ge(now - hours_since.second()))
            .load::<Crate>(conn)
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

pub struct UserRepository;

impl UserRepository {
    pub fn find(c: &mut PgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c)
    }

    pub fn find_by_username(c: &mut PgConnection, username: &String) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).first(c)
    }

    pub fn find_with_roles(
        c: &mut PgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load(c)?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)?
            .grouped_by(&users);

        Ok(users.into_iter().zip(result).collect())
    }

    pub fn create(
        conn: &mut PgConnection,
        new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(conn)?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(conn, &role_code) {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        name,
                        code: role_code,
                    };
                    let role = RoleRepository::create(conn, new_role)?;
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(conn)?;
        }

        Ok(user)
    }

    pub fn delete(conn: &mut PgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id))).execute(conn)?;

        diesel::delete(users::table.find(id)).execute(conn)
    }
}
pub struct RoleRepository;

impl RoleRepository {
    pub fn find_by_code(c: &mut PgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c)
    }

    pub fn find_by_ids(c: &mut PgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).get_results(c)
    }

    pub fn find_by_user(c: &mut PgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user).get_results(c)?;

        let role_ids = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        Self::find_by_ids(c, role_ids)
    }

    pub fn create(c: &mut PgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result::<Role>(c)
    }
}
