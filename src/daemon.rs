#[macro_use]
extern crate diesel;

mod db;
use crate::db::*;
mod models;
mod onvista;
mod schema;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let conn = establish_connection(&database_url);
    update_prices(&conn);
}
