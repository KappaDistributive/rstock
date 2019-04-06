use crate::models::Item;
use crate::onvista::*;
use crate::schema::prices;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn update_prices(conn: &PgConnection) {
    let watchlist = Item::all(conn);
    for item in watchlist {
        diesel::insert_into(prices::table)
            .values(&onvista_etf_now(item.isin))
            .execute(conn);
    }
}
