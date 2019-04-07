#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::State;
extern crate rocket_contrib;
use rocket_contrib::json::Json;

extern crate serde_derive;
#[macro_use]
extern crate serde_json;
use serde_json::Value;

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
fn latest(isin: String, state: State<Pool>) -> Json<Value> {
    match Price::newest_by_isin(isin, &state.get().expect("Couldn't connect to database.")) {
        Some(price) => Json(json!({
            "status": 200,
            "result": ResponsePrice::from_price(&price),
        })),
        None => Json(json!({
            "status": 404,
            "result": null,
        })),
    }
}

#[get("/<isin>")]
fn all_by_isin(isin: String, state: State<Pool>) -> Json<Value> {
    Json(json!({
        "status": 404,
        "result": ResponsePrice::from_prices(Price::all(Some(isin) , &state.get().expect("Couldn't connect to database."))),
    }))
}

#[get("/")]
fn all(state: State<Pool>) -> Json<Value> {
    Json(json!({
        "status": 404,
        "result": ResponsePrice::from_prices(Price::all(None,&state.get().expect("Couldn't connect to database."))),
    }))
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let pool = connect(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/rstock/all/", routes![all_by_isin])
        .launch();

    //    .mount("/rstock/latest/", routes![latest])
}
