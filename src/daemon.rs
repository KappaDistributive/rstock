#[macro_use]
extern crate diesel;
use bigdecimal::BigDecimal;
use chrono::{TimeZone, Utc};
use clap::{App, Arg, SubCommand};
use csv::ReaderBuilder;
use std::str::FromStr;

mod db;
use crate::db::{connect, insert_prices, update_prices};
mod models;
use models::NewPrice;
mod onvista;
mod schema;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Set DATABASE_URL");
    let conn = connect(database_url).get().unwrap();

    let matches = App::new("Rstock Daemon")
        .version("0.1")
        .subcommand(
            SubCommand::with_name("with_csv")
                .about("Adds provided prices to the price database.")
                .arg(
                    Arg::with_name("path")
                        .value_name("PATH")
                        .help("Provide stock price data in a csv.")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    // Data needs to be provided in the form of a csv whose rows are
    // of the following format:
    // <NAME>,<ISIN>,<KIND>,<%H:%M%:%S d%.m%.Y%> in UTC,<PRICE>,<CURRENCY>
    // There is no verfification process of the validity of provided data!
    if let Some(sc) = matches.subcommand_matches("with_csv") {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(sc.value_of("path").expect("Couldn't find csv."))
            .expect("Couldn't read csv.");
        let mut prices = Vec::<NewPrice>::new();

        for entry in reader.records() {
            match entry {
                Ok(column) => {
                    let mut data = Vec::<String>::new();
                    for item in column.iter() {
                        data.push(String::from(item));
                    }
                    println!("{:?}", data);
                    prices.push(NewPrice {
                        name: data[0].clone (),
                        isin: data[1].clone (),
                        kind: data[2].clone (),
                        time: Utc
                            .datetime_from_str(&data[3], "%H:%M:%S %d.%m.%Y")
                            .expect("Couldn't parse date.")
                            .with_timezone(&Utc),
                        price: BigDecimal::from_str(&data[4])
                            .expect("Couldn't parse price."),
                        currency: data[5].clone (),
                    });
                }
                Err(_) => println!("Missing entry!"),
            }
        }
        insert_prices(&conn,prices);
    } else {        
        update_prices(&conn);
    }
}
