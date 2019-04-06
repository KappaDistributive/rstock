#[macro_use]
extern crate diesel;

mod db;
use crate::db::*;
mod models;
use crate::models::*;
mod onvista;
mod schema;

fn main() {
    let conn = establish_connection();
    update_prices(&conn);
}
