use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;

use crate::schema::prices;
use crate::schema::prices::dsl::prices as all_prices;
use crate::schema::watchlist;
use crate::schema::watchlist::dsl::watchlist as all_watchlist;

#[derive(Queryable, Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Serialize)]
pub struct ResponsePrice {
    pub name: String,
    pub isin: String,
    pub kind: String,
    pub date: String,
    pub time: String,
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
    #[allow(dead_code)]
    pub fn all(isin: Option<String>, conn: &PgConnection) -> QueryResult<Vec<Price>> {
        match isin {
            Some(i) => all_prices
                .filter(prices::isin.eq(i))
                .order(prices::time.desc())
                .load::<Price>(conn),
            None => all_prices.order(prices::time.desc()).load::<Price>(conn),
        }
    }

    #[allow(dead_code)]
    pub fn find_by_id(id: i32, conn: &PgConnection) -> QueryResult<Price> {
        all_prices.find(id).load::<Price>(conn).and_then(|prices| {
            if prices.len() > 0 {
                Ok(prices[0].clone())
            } else {
                Err(Error::NotFound)
            }
        })
    }

    #[allow(dead_code)]
    pub fn newest_by_isin(isin: String, conn: &PgConnection) -> QueryResult<Price> {
        all_prices
            .filter(prices::isin.eq(isin))
            .load::<Price>(conn)
            .and_then(|prices| {
                if prices.len() > 0 {
                    Ok(prices[0].clone())
                } else {
                    Err(Error::NotFound)
                }
            })
    }
}

impl ResponsePrice {
    #[allow(dead_code)]
    pub fn from_price(price: &Price) -> ResponsePrice {
        ResponsePrice {
            name: price.name.clone(),
            isin: price.isin.clone(),
            kind: price.kind.clone(),
            date: price.time.format("%Y-%m-%d").to_string(),
            time: price.time.format("%H:%M:%S").to_string(),
            price: price.price.clone(),
            currency: price.currency.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn from_prices(prices: Vec<Price>) -> Vec<ResponsePrice> {
        prices
            .iter()
            .map(|p| ResponsePrice::from_price(&p))
            .collect()
    }
}

impl Item {
    #[allow(dead_code)]
    pub fn all(conn: &PgConnection) -> Vec<Item> {
        all_watchlist
            .order(watchlist::id.desc())
            .load::<Item>(conn)
            .expect("Error loading items")
    }

    #[allow(dead_code)]
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
