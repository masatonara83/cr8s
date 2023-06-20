use crate::auth;
use crate::models::{NewUser, RoleCode};
use crate::repositories::{CrateRepository, RoleRepository, UserRepository};

use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use std::str::FromStr;
use tera::{Context, Tera};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to postgres")
}

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parsing errors(s): {}", e);
    })
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

pub fn send_digest(to: String, hours_since: i32) {
    let mut c = load_db_connection();

    let crates = CrateRepository::find_since(&mut c, hours_since).unwrap();
    if crates.len() > 0 {
        println!("Sending thr digest for {} crates", crates.len());
        let tara = load_template_engine();
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);
        let html_body = tara.render("email/digest.html", &context).unwrap();

        //メール作成
        let message = lettre::Message::builder()
            .subject("Cr8s digest")
            .from("Cr8s <info@cr8s.com>".parse().unwrap())
            .to(to.parse().unwrap())
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .unwrap();
        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP url from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from env");
        let credentials = Credentials::new(smtp_username, smtp_password);
        let mailer = SmtpTransport::relay(&smtp_host)
            .unwrap()
            .credentials(credentials)
            .build();
        mailer.send(&message).unwrap();
    }
}
