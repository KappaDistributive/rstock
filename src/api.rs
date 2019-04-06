#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
use rocket_contrib::json::Json;
use serde_json::Value;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate diesel;

mod db;
use crate::db::*;
mod models;
use crate::models::*;
mod onvista;
mod schema;

use dotenv::dotenv;
use std::env;

#[get("/<isin>")]
fn newest(isin: String) -> Json<Value> {
    let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL");
    match Price::newest_by_isin(isin, &establish_connection(&database_url)) {
        // todo: make Serialize work for price and replace all of this
        Some(price) => Json(json!({
            "status": 200,
            "result": json!({
                "Name": price.name,
                "ISIN": price.isin,
                "Kind": price.kind,
                "Date": format!("{}",price.time.format("%Y-%m-%d")),
                "Time": format!("{}",price.time.format("%H:%M:%S %z")),
                "Price": format!("{}", price.price),
                "Currency": price.currency}),
        })),
        None => Json(json!({
            "status": 404,
            "result": null,
        })),
    }
}

fn main() {
    dotenv().ok();    
    rocket::ignite()
        .mount("/rstock/newest/", routes![newest])
        .launch();
}
