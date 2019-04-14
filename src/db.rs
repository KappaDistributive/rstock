use crate::models::{Item, NewPrice};
use crate::onvista::*;
use crate::schema::prices;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

#[allow (dead_code)]
pub fn connect(database_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new (database_url);
    Pool::new(manager).expect ("Failed to create pool.")
}

#[allow (dead_code)]
pub fn update_prices(conn: &PgConnection) {
    let watchlist = Item::all(conn);
    for item in watchlist {
        diesel::insert_into(prices::table)
            .values(&onvista_etf_now(item.isin))
            .execute(conn).ok();
    }
}

#[allow (dead_code)]
pub fn insert_prices(conn: &PgConnection, prices: Vec<NewPrice>) {
    for price in prices {
        diesel::insert_into(prices::table)
            .values(&price)
            .execute(conn).ok();
        println!("Inserted {:#?}", price);
    }
}
