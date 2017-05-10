#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::checks::dsl::*;

    let connection = establish_connection();
    create_check(&connection, "google.com", 60);
    let results = checks.limit(5)
        .load::<Check>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} checks", results.len());
    for check in results {
        println!("{}, {}", check.id, check.url);
    }
}

fn create_check<'a>(conn: &PgConnection, url: &'a str, rate: i32) -> Check {
    use schema::checks;

    let new_check = NewCheck {
        url: url,
        rate: rate,
    };

    diesel::insert(&new_check).into(checks::table)
        .get_result(conn)
        .expect("Error saving new post")
}