use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::prices;
use crate::schema::prices::dsl::prices as all_prices;
use crate::schema::watchlist;
use crate::schema::watchlist::dsl::watchlist as all_watchlist;

#[derive(Queryable, Debug, Clone)]
pub struct Price {
    pub id: i32,
    pub name: String,
    pub isin: String,
    pub kind: String,
    pub time: DateTime<Utc>,
    pub price: BigDecimal,
    pub currency: String,
}

#[derive(Insertable, Debug)]
#[table_name = "prices"]
pub struct NewPrice {
    pub name: String,
    pub isin: String,
    pub kind: String,
    pub time: DateTime<Utc>,
    pub price: BigDecimal,
    pub currency: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct Item {
    pub id: i32,
    pub isin: String,
    pub kind: String,
}

#[derive(Insertable)]
#[table_name = "watchlist"]
pub struct NewItem {
    pub isin: String,
    pub kind: String,
}

impl Price {
    pub fn find_by_id(id: i32, conn: &PgConnection) -> Option<Price> {
        let price_list = all_prices
            .find(id)
            .load::<Price>(conn)
            .expect("Error loading item");

        if price_list.len() > 0 {
            Some(price_list[0].clone())
        } else {
            None
        }
    }

    pub fn newest_by_isin(isin: String, conn: &PgConnection) -> Option<Price> {
        let price_list = all_prices
            .filter (prices::isin.eq (isin))
            .order(prices::time.desc())
            .load::<Price>(conn)
            .expect("Error loading prices");
        if price_list.len() > 0 {
            Some(price_list[0].clone ())
        } else {
            None
        }
    }
}

impl Item {
    pub fn all(conn: &PgConnection) -> Vec<Item> {
        all_watchlist
            .order(watchlist::id.desc())
            .load::<Item>(conn)
            .expect("Error loading items")
    }
    pub fn find_by_id(id: i32, conn: &PgConnection) -> Option<Item> {
        let item_list = all_watchlist
            .find(id)
            .load::<Item>(conn)
            .expect("Error loading item");

        if item_list.len() > 0 {
            Some(item_list[0].clone())
        } else {
            None
        }
    }
}