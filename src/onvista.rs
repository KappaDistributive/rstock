use select::document::Document;
use select::predicate::Name;

use bigdecimal::BigDecimal;
use chrono::Utc;
use std::str::FromStr;

use crate::models::NewPrice;

pub fn onvista_etf_price_label(isin: &String) -> (BigDecimal, String) {
    let res =
        reqwest::get(reqwest::Url::parse(&format!("https://www.onvista.de/etf/{}", isin)).unwrap())
            .expect("Unable to read response.");

    let price_label = Document::from_read(res)
        .expect("Unable to read response.")
        .find(Name("span"))
        .filter(|n| n.attr("class") == Some("price"))
        .next()
        .unwrap()
        .text()
        .split(" ")
        .map(|n| String::from(n))
        .collect::<Vec<String>>();

    let price: BigDecimal = BigDecimal::from_str(&price_label[0].replace(",", ".")).unwrap();
    let currency: String = price_label[1].clone();

    (price, currency)
}

pub fn onvista_etf_name(isin: &String) -> String {
    let res =
        reqwest::get(reqwest::Url::parse(&format!("https://www.onvista.de/etf/{}", isin)).unwrap())
            .expect("Unable to read response.");

    String::from(
        Document::from_read(res)
            .expect("Unable to read response.")
            .find(Name("title"))
            .next()
            .unwrap()
            .text()
            .split("-")
            .next()
            .unwrap()
            .trim(),
    )
}

pub fn onvista_etf_now(isin: String) -> NewPrice {
    let (price, currency) = onvista_etf_price_label(&isin);
    let name = onvista_etf_name(&isin);

    NewPrice {
        name: name,
        isin: isin,
        kind: String::from("ETF"),
        time: Utc::now(),
        price: price,
        currency: currency,
    }
}
