table! {
    prices (id) {
        id -> Int4,
        name -> Varchar,
        isin -> Varchar,
        kind -> Varchar,
        time -> Timestamptz,
        price -> Numeric,
        currency -> Varchar,
    }
}

table! {
    watchlist (id) {
        id -> Int4,
        isin -> Varchar,
        kind -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    prices,
    watchlist,
);
