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
    let conn = connect(database_url).get().unwrap ();
    update_prices(&conn);
}
