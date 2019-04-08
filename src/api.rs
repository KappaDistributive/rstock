#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::State;
extern crate rocket_contrib;
use rocket_contrib::json::Json;

extern crate serde_derive;

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
fn latest(isin: String, state: State<Pool>) -> Result<Json<Price>, Status> {
    match state.get() {
        Ok(conn) => match Price::newest_by_isin(isin, &conn) {
            Ok(price) => Ok(Json(ReponsePrice::from_price(price) )),
            Err(_error) => Err(Status::NotFound),
        },
        Err(_error) => Err(Status::InternalServerError),
    }
}

#[get("/<isin>")]
fn all_by_isin(isin: String, state: State<Pool>) -> Result<Json<Vec<Price>>, Status> {
    match state.get() {
        Ok(conn) => match Price::all(Some(isin), &conn) {
            Ok(prices) => Ok(Json(ResponsePrice::from_prices(prices))),
            Err(_error) => Err(Status::InternalServerError),
        },
        Err(_error) => Err(Status::InternalServerError),
    }
}

#[get("/")]
fn all(state: State<Pool>) -> Result<Json<Vec<Price>>, Status> {
    match state.get() {
        Ok(conn) => match Price::all(None, &conn) {
            Ok(prices) => Ok(Json(ResponsePrice::from_prices(prices))),
            Err(_error) => Err(Status::InternalServerError),
        },
        Err(_error) => Err(Status::InternalServerError),
    }
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let pool = connect(database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/rstock/all/", routes![all,all_by_isin])
        .mount("/rstock/latest/", routes![latest])
        .launch();

}
