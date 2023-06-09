use diesel::PgConnection;

mod models;
mod repositories;
mod rocket_routes;
mod schema;

#[rocket_sync_db_pools::database("postgres")]
pub struct DbConn(PgConnection);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                rocket_routes::rustaceans::get_rustaceans,
                rocket_routes::rustaceans::view_rustaceans,
                rocket_routes::rustaceans::create_rustacean,
                rocket_routes::rustaceans::update_rustacean,
                rocket_routes::rustaceans::delete_rustaceans,
            ],
        )
        .attach(DbConn::fairing())
        .launch()
        .await;
}
